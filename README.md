<!--
SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>

SPDX-License-Identifier: EUPL-1.2
-->

# SMTP Mailer

This is the [OpenTalk](https://opentalk.eu/) SMTP Mailer service. Its main purpose is to send out invites and updates on meetings.

## Development

There are two git submodules you need to checkout as a first step, to do so simply run:
```bash
git submodule init
git submodule update
```

## Deployment

To deploy the smtp mailer you can either use the Docker images provided here in Gitlab
or compile it yourself.  
The smtp-mailer looks for a configuration file called config.toml in the workdir. There is an example config file in this repository
where most of the configuration options are explained.

## Testing

The SMTP mailer includes a CLI utility to test its functionalities. It can either preview a generated mail by printing it to standard output or by sending a dummy invite via email.

The commands are `preview` and `preview-send` with the following options:

`preview`:
```
USAGE:
    smtp-mailer preview <TYPE> <TEMPLATE> <LANGUAGE>

ARGS:
    <TYPE>        Output type [possible values: html, plain]
    <TEMPLATE>    Template to preview [possible values: registered-invite, unregistered-invite,
                  external-invite, registered-cancellation, unregistered-cancellation,
                  external-cancellation]
    <LANGUAGE>    Language Code

OPTIONS:
    -h, --help    Print help information
```

`preview-send`:
```
USAGE:
    smtp-mailer preview-send <TEMPLATE> <TO>

ARGS:
    <TEMPLATE>    Template to preview [possible values: registered-invite, unregistered-invite,
                  external-invite, registered-cancellation, unregistered-cancellation,
                  external-cancellation]
    <TO>          To Email

OPTIONS:
    -h, --help    Print help information
```
