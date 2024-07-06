---
sidebar_position: 2
---

# RabbitMQ

When started up, the OpenTalk SMTP Mailer connects to the configured
[RabbitMQ](https://www.rabbitmq.com/) queue and consumes the messages it
receives from there. These messages must follow the definition of the
[OpenTalk Mail Worker Protocol](https://docs.opentalk.eu/developer/controller/mail_worker_protocol/).

## Configuration

The section in the [configuration file](configuration.md) is called `rabbit_mq`.

| Field                  | Type     | Required | Default value                       | Description                                        |
| ---------------------- | -------- | -------- | ----------------------------------- | -------------------------------------------------- |
| `url`                  | `string` | no       | "amqp://guest:guest@localhost:5672" | The RabbitMQ broker URL                            |
| `mail_task_queue`      | `string` | yes      | -                                   | The name of the RabbitMQ queue for the SMTP mailer |

### Example

```toml
[rabbit_mq]
url = "amqp://username:password@localhost/%2F"
mail_task_queue = "opentalk_mailer"
```
