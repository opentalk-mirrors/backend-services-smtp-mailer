# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://git.opentalk.dev/opentalk/smtp-mailer/-/compare/v0.4.3...main

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
