# Deploy

We can now build, push, and start the service:

```shell
{{#include ../../build_all.sh:birthday_server}}
```

In another terminal, check that the service runs:

```shell
{{#include ../../build_all.sh:service_check_birthday_server}}
```

```text
Service birthdayservice: found
```

You can also call the service with `service call`:

```shell
{{#include ../../build_all.sh:service_call_birthday_server}}
```

```text
Result: Parcel(
  0x00000000: 00000000 00000036 00610048 00700070 '....6...H.a.p.p.'
  0x00000010: 00200079 00690042 00740072 00640068 'y. .B.i.r.t.h.d.'
  0x00000020: 00790061 00420020 0062006f 0020002c 'a.y. .B.o.b.,. .'
  0x00000030: 006f0063 0067006e 00610072 00750074 'c.o.n.g.r.a.t.u.'
  0x00000040: 0061006c 00690074 006e006f 00200073 'l.a.t.i.o.n.s. .'
  0x00000050: 00690077 00680074 00740020 00650068 'w.i.t.h. .t.h.e.'
  0x00000060: 00320020 00200034 00650079 00720061 ' .2.4. .y.e.a.r.'
  0x00000070: 00210073 00000000                   's.!.....        ')
```
