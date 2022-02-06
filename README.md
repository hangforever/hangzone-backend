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

## General Project Structure

As I don't really know what I'm doing, the general project structure will be based off of [this "realworld" example](https://github.com/TatriX/realworld-rust-rocket). In their words:

> This codebase was created to demonstrate a fully fledged fullstack application built with Rocket including CRUD operations, authentication, routing, pagination, and more.
> 
> We've gone to great lengths to adhere to the Rocket community styleguides & best practices.

The only change is that I'll be foregoing the ORM and using the sqlx driver for postgres directly.

## DB

DB docs can be found [here](https://lucid.app/lucidchart/5d1620db-1277-483c-a043-5c55d07d27b0/edit?beaconFlowId=734CF7BD3888B9F4&page=0_0&invitationId=inv_d6a8c826-0bbb-4b38-81c8-d77e7003e9a0#)
