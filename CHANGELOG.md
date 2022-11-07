Changelog
---------

## Unreleased

### Added

- Added support for `...EventUpdate` and `...EventCancellation` messages
- First name and last name are now available for unregistered users as well
- Template variables for unregistered and external invitees changed

## 1.0.0-rc.2 (20 July, 2022)

### Fixed

- Improved outlook support by fixing the head section and adding special http-equiv meta tags (heinlein-video/smtp-mailer!18)
- Added timezone information to ics files
- Fixed missing newline in call-in sections
- Added timezone to dateformatter to return the correct time
- disabled bidi isolation unicode characters in fluent output

## 1.0.0-rc.1 (23 June, 2022)

initial release candidate
