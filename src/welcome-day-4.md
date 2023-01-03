# Welcome to Day 4

오늘 강의는 두가지 주제를 다룹니다: 
* 동시성: 쓰레드, 채널, 상태 공유(쉐어 스테이트), `Send`와 `Sync`
* 안드로이드: 바이너리와 라이브러리 작성, AIDL 사용, 로깅과 C/C++, 자바와의 상호 운용성

> 우리는 오늘 당신의 프로젝트를 호출해볼 것입니다. 따라서 가지고 있는 코드 중에 작은 부분을 찾아보세요
> 종속성이 적고 특이 타입이 적을 수록 좋습니다. raw byte를 분석하는 종류의 것이 이상적입니다.



> Today we will look at two main topics:
> 
> * Concurrency: threads, channels, shared state, `Send` and `Sync`.
> 
> * Android: building binaries and libraries, using AIDL, logging, and
>   interoperability with C, C++, and Java.
> 
> > We will attempt to call Rust from one of your own projects today. So try to
> > find a little corner of your code base where we can move some lines of code to
> > Rust. The fewer dependencies and "exotic" types the better. Something that
> > parses some raw bytes would be ideal.