# Running the Course

> This page is for the course instructor.

Here is a bit of background information about how we've been running the course
internally at Google.

To run the course, you need to:

1. Make yourself familiar with the course material. We've included speaker notes
   on some of the pages to help highlight the key points (please help us by
   contributing more speaker notes!). You should make sure to open the speaker
   notes in a popup (click the link with a little arrow next to "Speaker
   Notes"). This way you have a clean screen to present to the class.

2. Decide on the dates. Since the course is large, we recommend that you
   schedule the four days over two weeks. Course participants have said that
   they find it helpful to have a gap in the course since it helps them process
   all the information we give them.

3. Find a room large enough for your in-person participants. We recommend a
   class size of 15-20 people. That's small enough that people are comfortable
   asking questions --- it's also small enough that one instructor will have
   time to answer the questions.

4. On the day of your course, show up to the room a little early to set things
   up. We recommend presenting directly using `mdbook serve` running on your
   laptop. This ensures optimal performance with no lag as you change pages.
   Using your laptop will also allow you to fix typos as you or the course
   participants spot them.

5. Let people solve the exercises by themselves or in small groups. Make sure to
   ask people if they're stuck or if there is anything you can help with. When
   you see that several people have the same problem, call it out to the class
   and offer a solution, e.g., by showing people where to find the relvant
   information in the standard library.

6. If you don't skip the Android specific parts on Day 4, you will need an [AOSP
   checkout][1]. Make a checkout of the [course repository][2] on the same
   machine and move the `src/android/` directory into the root of your AOSP
   checkout. This will ensure that the Android build system sees the
   `Android.bp` files in `src/android/`.

   Ensure that `adb sync` works with your emulator or real device and pre-build
   all Android examples using `src/android/build_all.sh`. Read the script to see
   the commands it runs and make sure they work when you run them by hand.

That is all, good luck running the course! We hope it will be as much fun for
you as it has been for us!

Please [provide feedback][3] afterwards so that we can keep improving the
course. We would love to hear what worked well for you and what can be made
better. Your students are also very welcome to [send us feedback][4]!

[1]: https://source.android.com/docs/setup/download/downloading
[2]: https://github.com/google/comprehensive-rust
[3]: https://github.com/google/comprehensive-rust/discussions/86
[4]: https://github.com/google/comprehensive-rust/discussions/100
