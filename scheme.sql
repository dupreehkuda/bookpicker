CREATE TABLE "bookclub" (
                            "chat_id" integer PRIMARY KEY,
                            "last_event" timestamp,
                            "next_event" timestamp,
                            "created_at" timestamp,
                            "active_event" uuid
);

CREATE TABLE "events" (
                          "id" uuid PRIMARY KEY,
                          "chat_id" integer,
                          "book" text,
                          "who_suggested" integer,
                          "created_at" timestamp
);

CREATE TABLE "suggestions" (
                               "event_id" uuid,
                               "chat_id" integer,
                               "user_id" integer,
                               "suggestion" text,
                               "created_at" timestamp
);

ALTER TABLE "events" ADD FOREIGN KEY ("chat_id") REFERENCES "bookclub" ("chat_id");
ALTER TABLE "suggestions" ADD FOREIGN KEY ("event_id") REFERENCES "events" ("id");
