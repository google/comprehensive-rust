<!--
Copyright 2024 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Updating Client and Service

Update the client and server code to account for the new API.

_birthday_service/src/lib.rs_:

```rust,ignore
# // Copyright 2024 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
impl IBirthdayService for BirthdayService {
    fn wishHappyBirthday(
        &self,
        name: &str,
        years: i32,
        text: &[String],
    ) -> binder::Result<String> {
        let mut msg = format!(
            "Happy Birthday {name}, congratulations with the {years} years!",
        );

        for line in text {
            msg.push('\n');
            msg.push_str(line);
        }

        Ok(msg)
    }
}
```

_birthday_service/src/client.rs_:

```rust,ignore
# // Copyright 2024 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
let msg = service.wishHappyBirthday(
    &name,
    years,
    &[
        String::from("Habby birfday to yuuuuu"),
        String::from("And also: many more"),
    ],
)?;
```

<details>

- TODO: Move code snippets into project files where they'll actually be built?

</details>
