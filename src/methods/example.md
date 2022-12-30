# Example

```rust,editable
#[derive(Debug)]
struct Race {
    name: String,
    laps: Vec<i32>,
}

impl Race {
    // 이 메서드는 self가 없습니다. 이것은 정적 메서드 입니다. 
    // No receiver, a static method
    fn new(name: &str) -> Race {  
        Race { name: String::from(name), laps: Vec::new() }
    }

    // 이 메서드는 self에 대한 독점적인 읽기-쓰기 권한을 갖습니다.
    // Exclusive borrowed read-write access to self
    fn add_lap(&mut self, lap: i32) {  
        self.laps.push(lap);
    }

    // 이 메서드는 self 대한 공유 및 읽기전용 빌림 권한을 갖습니다.
    // Shared and read-only borrowed access to self
    fn print_laps(&self) {  
        println!("Recorded {} laps for {}:", self.laps.len(), self.name);
        for (idx, lap) in self.laps.iter().enumerate() {
            println!("Lap {idx}: {lap} sec");
        }
    }

    // 이 메서드는 self의 소유권을 가져옵니다. 
    // self를 반환하지 않기 때문에 이 메서드가 호출 된 후 main에서 race 재사용 할 수 없습니다.
    // Exclusive ownership of self
    fn finish(self) {  
        let total = self.laps.iter().sum::<i32>();
        println!("Race {} is finished, total lap time: {}", self.name, total);
    }
}

fn main() {
    let mut race = Race::new("Monaco Grand Prix");
    race.add_lap(70);
    race.add_lap(68);
    race.print_laps();
    race.add_lap(71);
    race.print_laps();
    race.finish();
}
```
