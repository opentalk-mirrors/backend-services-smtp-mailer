use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use ics::components::Property;
use ics::properties::{Attendee, Description, ExDate, Organizer, RDate, RRule, Summary};
use ics::{escape_text, parameters, Event, ICalendar};
use ics_chrono_tz::ToIcsTimeZone;
use mail_worker_protocol::v1;

pub enum Invitee<'a> {
    WithName { email: &'a str, name: &'a str },
    WithoutName(&'a str),
}

pub(crate) fn create_ics_v1(
    inviter: &v1::User,
    event: &v1::Event,
    invitee: Invitee,
) -> Result<Option<Vec<u8>>> {
    let (start, end) = if let (Some(start), Some(end)) = (&event.start_time, &event.end_time) {
        (start, end)
    } else {
        return Ok(None);
    };

    let mut calendar = ICalendar::new("2.0", "-//OpenTalk GmbH//NONSGML smtp-mailer v1.0//EN");

    let start_tz: Tz = start.timezone.parse().ok().context("start timezone")?;
    let end_tz: Tz = end.timezone.parse().ok().context("end timezone")?;

    calendar.add_timezone(ToIcsTimeZone::to_latest_ics_timezone(&start_tz));

    if start_tz != end_tz {
        calendar.add_timezone(ToIcsTimeZone::to_latest_ics_timezone(&end_tz));
    }

    // create event which contains the information regarding the conference
    let mut event_obj = Event::new(
        event.id.to_string(),
        Utc::now().format("%Y%m%dT%H%M%SZ").to_string(),
    );

    // add properties
    event_obj.push(Summary::new(escape_text(event.name.clone())));
    event_obj.push(Description::new(escape_text(event.description.clone())));

    let mut organizer_property = Organizer::new(format!("mailto:{}", inviter.email.as_ref()));
    organizer_property
        .append(parameters!("CN" => format!("{} {}", inviter.first_name, inviter.last_name)));
    event_obj.push(organizer_property);

    match invitee {
        Invitee::WithName { email, name } => {
            let mut attendee_property = Attendee::new(format!("mailto:{}", email));
            attendee_property.append(parameters!("CN" => name));
            event_obj.push(attendee_property);
        }
        Invitee::WithoutName(email) => {
            let attendee_property = Attendee::new(format!("mailto:{}", email));
            event_obj.push(attendee_property);
        }
    }

    let dt_start = create_time_property("DTSTART", start.time, &start.timezone);
    event_obj.push(dt_start.clone());

    if let Some(rrule) = &event.rrule {
        // Todo: The Display implementation of the rrule crate is not really usable.
        // Therefore we rely on the verification of the web-api

        for sigment in rrule.split('\n') {
            if sigment.starts_with("RRULE:") {
                let rrule_property = RRule::new(sigment.trim_start_matches("RRULE:"));
                event_obj.push(rrule_property);
            } else if sigment.starts_with("EXDATE:") {
                let exdate_property = ExDate::new(sigment.trim_start_matches("EXDATE:"));
                event_obj.push(exdate_property);
            } else if sigment.starts_with("RDATE:") {
                let rdate_property = RDate::new(sigment.trim_start_matches("RDATE:"));
                event_obj.push(rdate_property);
            }
        }
    }

    if let Some(end_time) = &event.end_time {
        event_obj.push(create_time_property(
            "DTEND",
            end_time.time,
            &end_time.timezone,
        ));
    }

    calendar.add_event(event_obj);

    let mut buf = Vec::new();
    calendar.write(&mut buf)?;
    Ok(Some(buf))
}

fn create_time_property<'a>(
    property_name: &'a str,
    time: DateTime<Utc>,
    timezone: &'a str,
) -> Property<'a> {
    match timezone.parse::<Tz>().ok() {
        Some(Tz::UTC) | None => {
            Property::new(property_name, time.format("%Y%m%dT%H%M%SZ").to_string())
        }
        Some(tz) => {
            let start_time = time.with_timezone(&tz);
            let mut prop = Property::new(
                property_name,
                start_time.format("%Y%m%dT%H%M%S").to_string(),
            );
            prop.append(parameters!("TZID" => timezone));
            prop
        }
    }
}

#[cfg(test)]
mod test {
    use super::{create_ics_v1, Invitee};
    use chrono::{TimeZone, Utc};
    use mail_worker_protocol::v1::{CallIn, Event, Room, Time, User};
    use uuid::Uuid;

    #[test]
    fn test_ics() {
        let user = User {
            email: "klaus@example.org".into(),
            title: "Dr.".to_owned(),
            first_name: "Klaus".to_owned(),
            last_name: "Doktor".to_owned(),
            language: "de_DE".to_owned(),
        };

        let event = Event {
            id: Uuid::from_u128(2),
            name: "Test".to_owned(),
            start_time: Some(Time {
                time: Utc.ymd(2022, 6, 20).and_hms(0, 0, 0),
                timezone: "Europe/Berlin".to_owned(),
            }),
            end_time: Some(Time {
                time: Utc.ymd(2022, 6, 20).and_hms(10, 0, 0),
                timezone: "Europe/Berlin".to_owned(),
            }),
            rrule: Some("RRULE:FREQ=WEEKLY;COUNT=30;INTERVAL=1".to_owned()),
            call_in: Some(CallIn {
                sip_tel: "+4980011880".to_owned(),
                sip_id: "0123456789".to_owned(),
                sip_password: "555NASE".to_owned(),
            }),
            description: "Lel".to_owned(),
            room: Room {
                id: Uuid::from_u128(3),
                password: Some("ddd".to_owned()),
            },
        };

        let invitee = Invitee::WithName {
            email: "g@example.org",
            name: "G. G.",
        };
        let ics = create_ics_v1(&user, &event, invitee).unwrap().unwrap();
        let ics_str = String::from_utf8_lossy(&ics);
        let to_test = ics_str.lines().collect::<Vec<_>>();

        assert!(to_test.contains(&"BEGIN:VEVENT"));
        assert!(to_test.contains(&"ATTENDEE;CN=G. G.:mailto:g@example.org"));
        assert!(to_test.contains(&"ORGANIZER;CN=Klaus Doktor:mailto:klaus@example.org"));
        assert!(to_test.contains(&"SUMMARY:Test"));
        assert!(to_test.contains(&"DESCRIPTION:Lel"));
        assert!(to_test.contains(&"DTSTART;TZID=Europe/Berlin:20220620T020000"));
        assert!(to_test.contains(&"DTEND;TZID=Europe/Berlin:20220620T120000"));
        assert!(to_test.contains(&"RRULE:FREQ=WEEKLY;COUNT=30;INTERVAL=1"));
    }
}
