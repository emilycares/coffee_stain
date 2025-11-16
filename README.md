# coffee_stain
## Features
- hint
- code

## Usage
- logana In https://github.com/emilycares/logana the better error message will be appended after the original one.
- cli

## hint
### Example
More examples can be found in the tests of https://github.com/emilycares/coffee_stain/blob/main/src/lib.rs
``` java
@Test
void list() {
    var a = new User("first", null);
    var b = new User("second", null);
    assertEquals(List.of(a), List.of(a, b));
}
```
``` text
org.opentest4j.AssertionFailedError: expected: <[User(name=first, other=null)]> but was: <[User(name=first, other=null), User(name=second, other=null)]>

coffee_stain output " -> [ additional User(name=\"second\",other=null)]"
```
## code
