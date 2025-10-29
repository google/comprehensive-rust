# Running the Course

> This page is for the course instructor.

Here is a bit of background information about how we've been running the course
internally at Google.

We typically run classes from 9:00 am to 4:00 pm, with a 1 hour lunch break in
the middle. This leaves 3 hours for the morning class and 3 hours for the
afternoon class. Both sessions contain multiple breaks and time for students to
work on exercises.

Before you run the course, you will want to:

1. Make yourself familiar with the course material. We've included speaker notes
   to help highlight the key points (please help us by contributing more speaker
   notes!). When presenting, you should make sure to open the speaker notes in a
   popup (click the link with a little arrow next to "Speaker Notes"). This way
   you have a clean screen to present to the class.

1. Decide on the dates. Since the course takes four days, we recommend that you
   schedule the days over two weeks. Course participants have said that they
   find it helpful to have a gap in the course since it helps them process all
   the information we give them.

1. Find a room large enough for your in-person participants. We recommend a
   class size of 15-25 people. That's small enough that people are comfortable
   asking questions --- it's also small enough that one instructor will have
   time to answer the questions. Make sure the room has _desks_ for yourself and
   for the students: you will all need to be able to sit and work with your
   laptops. In particular, you will be doing a lot of live-coding as an
   instructor, so a lectern won't be very helpful for you.

1. On the day of your course, show up to the room a little early to set things
   up. We recommend presenting directly using `mdbook serve` running on your
   laptop (see the [installation instructions][3]). This ensures optimal
   performance with no lag as you change pages. Using your laptop will also
   allow you to fix typos as you or the course participants spot them.

1. Let people solve the exercises by themselves or in small groups. We typically
   spend 30-45 minutes on exercises in the morning and in the afternoon
   (including time to review the solutions). Make sure to ask people if they're
   stuck or if there is anything you can help with. When you see that several
   people have the same problem, call it out to the class and offer a solution,
   e.g., by showing people where to find the relevant information in the
   standard library.

That is all, good luck running the course! We hope it will be as much fun for
you as it has been for us!

Please [provide feedback][1] afterwards so that we can keep improving the
course. We would love to hear what worked well for you and what can be made
better. Your students are also very welcome to [send us feedback][2]!

[1]: https://github.com/google/comprehensive-rust/discussions/86
[2]: https://github.com/google/comprehensive-rust/discussions/100
[3]: https://github.com/google/comprehensive-rust#building
[red-box]: ?show-red-box=true

<details>

### Instructor Preparation

- **Go through all the material:** Before teaching the course, make sure you
  have gone through all the slides and exercises yourself. This will help you
  anticipate questions and potential difficulties.
- **Prepare for live coding:** The course involves a lot of live coding.
  Practice the examples and exercises beforehand to ensure you can type them out
  smoothly during the class. Have the solutions ready in case you get stuck.
- **Familiarize yourself with `mdbook`:** The course is presented using
  `mdbook`. Knowing how to navigate, search, and use its features will make the
  presentation smoother.
- **Slice size helper:** Press <kbd>Ctrl</kbd> + <kbd>Alt</kbd> + <kbd>B</kbd>
  to toggle a visual guide showing the amount of space available when
  presenting. Expect any content outside of the red box to be hidden initially.
  Use this as a guide when editing slides. You can also
  [enable it via this link][red-box].

### Creating a Good Learning Environment

- **Encourage questions:** Reiterate that there are no "stupid" questions. A
  welcoming atmosphere for questions is crucial for learning.
- **Manage time effectively:** Keep an eye on the schedule, but be flexible.
  It's more important that students understand the concepts than sticking
  rigidly to the timeline.
- **Facilitate group work:** During exercises, encourage students to work
  together. This can help them learn from each other and feel less stuck.

</details>
