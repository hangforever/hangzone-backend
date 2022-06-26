/* hang sessions */
insert into hang_sessions (name, description, hangzone_id) values ('Holdlamis', 'Variant Creutzfeldt-Jakob disease', 2);
insert into hang_sessions (name, description, hangzone_id) values ('Sonsing', 'Combined forms of age-related cataract, left eye', 3);
insert into hang_sessions (name, description, hangzone_id) values ('Stronghold', 'Poisoning by, adverse effect of and underdosing of vitamins', 4);
insert into hang_sessions (name, description, hangzone_id) values ('Solarbreeze', 'Other specified rheumatoid arthritis, right ankle and foot', 5);
insert into hang_sessions (name, description, hangzone_id) values ('Andalax', 'Motorcycle passenger injured in collision with fixed or stationary object in nontraffic accident, subsequent encounter', 6);

/* hangers */
insert into hangers (hang_session_id, user_hanger_id, host) values (1, 1, true);
insert into hangers (hang_session_id, user_hanger_id, host) values (2, 2, true);
insert into hangers (hang_session_id, user_hanger_id, host) values (3, 3, true);
insert into hangers (hang_session_id, user_hanger_id, host) values (4, 4, true);
insert into hangers (hang_session_id, user_hanger_id, host) values (5, 5, true);
