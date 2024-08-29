# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.8.0]

### <!-- 0 -->:bug: Bug Fixes

- *(deps)* Update rust crate async-trait to v0.1.81
- *(deps)* Update rust crate serde to v1.0.204
- *(deps)* Update rust crate clap to v4.5.9
- *(deps)* Update rust crate uuid to v1.10.0
- *(deps)* Update rust crate phonenumber to v0.3.6
- *(deps)* Update rust crate thiserror to v1.0.62
- *(deps)* Update rust crate bytes to v1.6.1
- *(deps)* Update rust crate tokio to v1.38.1
- *(deps)* Update rust crate lapin to v2.4.0
- *(deps)* Update rust crate thiserror to v1.0.63

### <!-- 1 -->:books: Documentation

- Add a general description and link the protocol docs

## [0.7.0]

### <!-- 0 -->:rocket: New features

- Add event details to email templates ([#85](https://git.opentalk.dev/opentalk/backend/services/smtp-mailer/-/issues/85))

### <!-- 3 -->:gear: Dependencies

- Update controller submodule to version 0.16.0-rc.1
- Update git.opentalk.dev:5050/opentalk/backend/containers/rust docker tag to v1.79.0
- Update rust crate anyhow to v1.0.86
- Update rust crate chrono-tz to 0.9
- Update rust crate clap to v4.5.8
- Update rust crate css-inline to 0.14.0
- Update rust crate fluent-templates to 0.9.0
- Update rust crate ics-chrono-tz to 0.3
- Update rust crate lapin to v2.3.4
- Update rust crate lettre to 0.11.0
- Update rust crate log to v0.4.22
- Update rust crate phonenumber to v0.3.5
- Update rust crate serde to v1.0.203
- Update rust crate serde_json to v1.0.119
- Update rust crate tera to v1.20.0
- Update rust crate thiserror to v1.0.61
- Update rust crate tokio to v1.38.0
- Update rust crate url to v2.5.2
- Update rust crate uuid to v1.9.1

### Ci

- Lint commits, md, yaml and format tomls

## [0.6.0]

### :rocket: New features

- Added previews for: registered-event-update and registered-uninvite
- Add streaming information to all templates ([#66](https://git.opentalk.dev/opentalk/backend/services/smtp-mailer/-/issues/66))

### :bug: Bug fixes

- Don't show passwords for registered users ([#93](https://git.opentalk.dev/opentalk/backend/services/smtp-mailer/-/issues/93))
- print Version output ([!92](https://git.opentalk.dev/opentalk/backend/services/smtp-mailer/-/merge_requests/91))

### Ci

- Upgrade debian image in ci & container creation to bookworm ([#92](https://git.opentalk.dev/opentalk/backend/services/smtp-mailer/-/issues/92))

### :gear: Miscellaneous Tasks

- Update controller submodule to v0.15.0

## [0.5.2]

### Fixed

- `RUSTSEC-2024-0336` by updating dependencies ([#94](https://git.opentalk.dev/opentalk/backend/services/smtp-mailer/-/issues/94))

## [0.5.1]

### Changed

- Update dependencies ([#88](https://git.opentalk.dev/opentalk/backend/services/smtp-mailer/-/issues/88))
- Use ics-chrono-tz from crates.io ([#87](https://git.opentalk.dev/opentalk/backend/services/smtp-mailer/-/issues/87))

## [0.5.0] - 2024-02-22

### Changed

- Create a proper ICS file for event instance updates ([#60](https://git.opentalk.dev/opentalk/backend/services/smtp-mailer/-/issues/60))

## [0.4.3] - 2024-01-10

### Added

- Add deletion notice for adhoc meetings to emails ([#57](https://git.opentalk.dev/opentalk/backend/services/smtp-mailer/-/issues/57))

### Fixed

- Update `self_cell` dependency to fix [`RUSTSEC-2023-0070`](https://rustsec.org/advisories/RUSTSEC-2023-0070) ([#76](https://git.opentalk.dev/opentalk/backend/services/smtp-mailer/-/issues/76))

## [0.4.2] - 2023-11-02

### Fixed

- Pin dockerfile build container to Debian Bullseye to fix GLIBC errors ([#72](https://git.opentalk.dev/opentalk/backend/services/smtp-mailer/-/issues/72))

## [0.4.1] - 2023-11-02

### Fixed

- Pin build base container to Debian Bullseye to fix GLIBC errors ([#70](https://git.opentalk.dev/opentalk/backend/services/smtp-mailer/-/issues/70))

## [0.4.0] - 2023-10-30

### Added

- Add quick guide hint (including link) to the generated emails ([#65](https://git.opentalk.dev/opentalk/smtp-mailer/-/issues/65))
- Send emails to users when they are removed from a meeting ([#48](https://git.opentalk.dev/opentalk/smtp-mailer/-/issues/48))

### Changed

- Change wording in emails to match the voice announcements ([#46](https://git.opentalk.dev/opentalk/smtp-mailer/-/issues/46))
- Add quick guide hint (including link) to the generated emails ([#65](https://git.opentalk.dev/opentalk/smtp-mailer/-/issues/65))
- Update CI rust tooling to 1.73.0  ([#62](https://git.opentalk.dev/opentalk/smtp-mailer/-/issues/62))

## [0.3.0] - 2023-06-27

### Added

- Include shared folder URL and password in emails ([#34](https://git.opentalk.dev/opentalk/smtp-mailer/-/issues/34))
- Data protection hints ([#39](https://git.opentalk.dev/opentalk/smtp-mailer/-/issues/39))

### Fixed

- Make shared folder link clickable ([#41](https://git.opentalk.dev/opentalk/smtp-mailer/-/issues/41))
- Replace deprecated lobby URLs with room counterparts ([#61](https://git.opentalk.dev/opentalk/smtp-mailer/-/issues/61))

## [0.2.0] - 2023-05-15

### Changed

- The creator is now referenced via Reply-To instead of From header field, From is configurable ([#33](https://git.opentalk.dev/opentalk/smtp-mailer/-/issues/33))

### Fixed

- Show the inviter's full name, not just the first name ([#31](https://git.opentalk.dev/opentalk/smtp-mailer/-/issues/31))

## [0.1.0] - 2023-03-01

### Added

- Add license information

### Fixed

- fixed a bug where environment variables did not overwrite config values

## [0.0.0-internal-release.4] - 2022-11-10

### Added

- Added support for `...EventUpdate` and `...EventCancellation` messages
- First name and last name are now available for unregistered users as well

### Changed

- changed internationalization strings for unregistered and external invitees
- exit process on IO errors or when the SMTP-server is no longer available

### Fixed

- do not requeue emails when encountering permanent errors

## [0.0.0-internal-release.3] - 2022-07-20

### Changed

- internal dependency updates

## [0.0.0-internal-release.2] - 2022-07-20

### Fixed

- Improved outlook support by fixing the head section and adding special http-equiv meta tags (opentalk/smtp-mailer!18)
- Added timezone information to ics files
- Fixed missing newline in call-in sections
- Added timezone to dateformatter to return the correct time
- disabled bidi isolation unicode characters in fluent output

## [0.0.0-internal-release.1] - 2022-06-23

initial release candidate

[Unreleased]: https://git.opentalk.dev/opentalk/smtp-mailer/-/compare/v0.8.0...main

[0.8.0]: https://git.opentalk.dev/opentalk/smtp-mailer/-/compare/v0.7.0...v0.8.0

[0.7.0]: https://git.opentalk.dev/opentalk/smtp-mailer/-/compare/v0.6.0...v0.7.0

[0.6.0]: https://git.opentalk.dev/opentalk/smtp-mailer/-/compare/v0.5.2...v0.6.0

[0.5.2]: https://git.opentalk.dev/opentalk/smtp-mailer/-/compare/v0.5.1...v0.5.2
[0.5.1]: https://git.opentalk.dev/opentalk/smtp-mailer/-/compare/v0.5.0...v0.5.1
[0.5.0]: https://git.opentalk.dev/opentalk/smtp-mailer/-/compare/fb7bb1046c6850b5bf8f42f7ab58a4eab68d7ad3...v0.5.0

[0.4.3]: https://git.opentalk.dev/opentalk/backend/services/smtp-mailer/-/compare/v0.4.2...v0.4.3
[0.4.2]: https://git.opentalk.dev/opentalk/backend/services/smtp-mailer/-/compare/v0.4.1...v0.4.2
[0.4.1]: https://git.opentalk.dev/opentalk/backend/services/smtp-mailer/-/compare/v0.4.0...v0.4.1
[0.4.0]: https://git.opentalk.dev/opentalk/backend/services/smtp-mailer/-/compare/f51209ff8fc6d709c3df81198bf709b88a64f44d...v0.4.0

[0.3.0]: https://git.opentalk.dev/opentalk/smtp-mailer/-/compare/25b0d812a39f1cfb6b5dd11598a7be9e20964eda...v0.3.0

[0.2.0]: https://git.opentalk.dev/opentalk/smtp-mailer/-/compare/ffadbb4aacd48a91eb0553dba966745476640941...v0.2.0

[0.1.0]: https://git.opentalk.dev/opentalk/smtp-mailer/-/compare/14c20df5d3a54c39332287ebf719ec04b49d4bab...v0.1.0

[0.0.0-internal-release.4]: https://git.opentalk.dev/opentalk/smtp-mailer/-/compare/7579c621c3b08c086974e7ccf4365345762e69c8...14c20df5d3a54c39332287ebf719ec04b49d4bab
[0.0.0-internal-release.3]: https://git.opentalk.dev/opentalk/smtp-mailer/-/compare/8f7edbcb83ac2b38402ebc951f76b75ddc51043d...7579c621c3b08c086974e7ccf4365345762e69c8
[0.0.0-internal-release.2]: https://git.opentalk.dev/opentalk/smtp-mailer/-/compare/5de6fd35a071b05cd05fd03838d436bd9c79db53...8f7edbcb83ac2b38402ebc951f76b75ddc51043d
[0.0.0-internal-release.1]: https://git.opentalk.dev/opentalk/smtp-mailer/-/commits/5de6fd35a071b05cd05fd03838d436bd9c79db53
