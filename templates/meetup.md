## Rust Cloud Native online meetup #{{ meetup.id }}

*{{ meetup.date }}, {{ meetup.pdt_time }} PDT, {{ meetup.cet_time }} CET, {{ meetup.region }}-friendly*

Let's meet online to talk about:

* the present and the future of RCN
* projects and topics you're interested inn
* casual stuff afterwards

Meetup will take place on [our Discord server](https://discord.gg/799cmsYB4q)
on the `meetup` voice channel.

Feel free to submit a pull request to the [rust-cloud-native.github.io repo](https://github.com/rust-cloud-native/rust-cloud-native.github.io)
if you want to propose any agenda items.

Rust Cloud Native meetups are going to be organized every month. Each month
the time slot is going to change between more AMER-friendly and more
EMEA-friendly.

### Agenda

{% for agenda_item in meetup.agenda -%}
* {% if agenda_item.url %}**[{{ agenda_item.title }}]({{ agenda_item.url }})**{% else %}**{{ agenda_item.title }}**{% endif %} - {% if agenda_item.speaker_name %}{% if agenda_item.speaker_url %}([{{ agenda_item.speaker_name }}]({{ agenda_item.speaker_url }})) {% else %}({{ agenda_item.speaker_name }}) {% endif %}{% endif %}{{ agenda_item.description }}
{% endfor -%}
* *Feel free to submit your talk proposal here*
* **Networking** - casual talk, an opportunity to talk about interests and
connect.