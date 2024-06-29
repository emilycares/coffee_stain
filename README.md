# coffee_stain
A junit AssertionFailedError diff tool

## Example
More examples can be found in the tests of https://github.com/micmine/coffee_stain/blob/main/src/lib.rs
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
## Usage
### In Logana
In https://github.com/micmine/logana the better error message will be appended after the original one.
### In cli
1. Build
2. Run

``` text
Please paste the line that includes:
  - "org.opentest4j.AssertionFailedError: expected: <*> but was: <*>"
```
