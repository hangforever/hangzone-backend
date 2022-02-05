## Description

A monolithic server to handle all the backend needs for Hangzone.

Uses:

- JSON API server
  - hangzones
  - hangers
  - maps
- DB
  - Read/write
  - Migrations

Below is a tentative list of tools I plan to use

| Use                | Tool       |
| ------------------ | ---------- |
| HTTP               | Rocket     |
| JSON serialization | Serde      |
| DB                 | Postgresql |
| DB Driver          | sqlx       |

## DB

DB docs can be found [here](https://lucid.app/lucidchart/5d1620db-1277-483c-a043-5c55d07d27b0/edit?beaconFlowId=734CF7BD3888B9F4&page=0_0&invitationId=inv_d6a8c826-0bbb-4b38-81c8-d77e7003e9a0#)
