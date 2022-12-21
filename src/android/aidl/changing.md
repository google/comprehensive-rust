# Changing API

Let us extend the API with more functionality: we want to let clients specify a
list of lines for the birthday card:

```java
package com.example.birthdayservice;

/** Birthday service interface. */
interface IBirthdayService {
    /** Generate a Happy Birthday message. */
    String wishHappyBirthday(String name, int years, in String[] text);
}
```
