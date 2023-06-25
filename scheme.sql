CREATE TABLE "bookclub" (
                            "chat_id" int8 PRIMARY KEY NOT NULL,
                            "last_event" timestamp,
                            "next_event" timestamp,
                            "created_at" timestamptz NOT NULL DEFAULT NOW() ,
                            "active_event" uuid
);

CREATE TABLE "events" (
                          "id" uuid PRIMARY KEY NOT NULL,
                          "chat_id" int8 NOT NULL,
                          "book" text,
                          "who_suggested" integer,
                          "event_date" timestamptz NOT NULL,
                          "created_at" timestamptz NOT NULL DEFAULT NOW()
);

CREATE TABLE "suggestions" (
                               "event_id" uuid NOT NULL,
                               "chat_id" int8 NOT NULL,
                               "user_id" int8 NOT NULL,
                               "suggestion" text,
                               "created_at" timestamptz NOT NULL DEFAULT NOW()
);

ALTER TABLE "events" ADD FOREIGN KEY ("chat_id") REFERENCES "bookclub" ("chat_id");
ALTER TABLE "suggestions" ADD FOREIGN KEY ("event_id") REFERENCES "events" ("id");
