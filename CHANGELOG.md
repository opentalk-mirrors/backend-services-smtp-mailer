# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0-rc.4] 2022-11-10

### Added

- Added support for `...EventUpdate` and `...EventCancellation` messages
- First name and last name are now available for unregistered users as well

### Changed

- changed internationalization strings for unregistered and external invitees
- exit process on IO errors or when the SMTP-server is no longer available

### Fixed

- do not requeue emails when encountering permanent errors

## [1.0.0-rc.3] 2022-07-20

### Changed

- internal dependency updates

## [1.0.0-rc.2] 2022-07-20

### Fixed

- Improved outlook support by fixing the head section and adding special http-equiv meta tags (heinlein-video/smtp-mailer!18)
- Added timezone information to ics files
- Fixed missing newline in call-in sections
- Added timezone to dateformatter to return the correct time
- disabled bidi isolation unicode characters in fluent output

## [1.0.0-rc.1] 2022-06-23

initial release candidate

[Unreleased]: https://git.heinlein-video.de/heinlein-video/smtp-mailer/-/compare/v1.0.0-rc.4...main
[1.0.0-rc.4]: https://git.heinlein-video.de/heinlein-video/smtp-mailer/-/compare/v1.0.0-rc.3...v1.0.0-rc.4
[1.0.0-rc.3]: https://git.heinlein-video.de/heinlein-video/smtp-mailer/-/compare/v1.0.0-rc.2...v1.0.0-rc.3
[1.0.0-rc.2]: https://git.heinlein-video.de/heinlein-video/smtp-mailer/-/compare/v1.0.0-rc.1...v1.0.0-rc.2
[1.0.0-rc.1]: https://git.heinlein-video.de/heinlein-video/smtp-mailer/-/commits/v1.0.0-rc.1
