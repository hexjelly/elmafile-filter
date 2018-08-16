# Change Log

All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## \[0.1.13\] - 2018-08-16

### Added

-   Level's object struct now has `.is_player()` and `is_apple()` methods, thanks to Smibu.

### Breaking

-   All main structs now keep track of file paths in a `path` field.

## \[0.1.12\] - 2018-07-17

### Added

-   Added full state.dat support, once again thanks to Smibu <3.

## \[0.1.11\] - 2018-07-14

### Fixed

-   Replays can now be accurately loaded/saved on a byte level thanks to Smibu <3.

### Added

-   Added `.to_bytes()` for `Replay` struct.

## \[0.1.10\] - 2018-05-15

### Breaking

-   `to_parts()` on `Time` now returns tuple with `bool` signifying positive
or negative number, along with positive integers for all parts.

## \[0.1.9\] - 2018-05-12

### Added

-   LGR reading and writing.
-   Impl Deref to i32 for `Time`.

## \[0.1.8\] - 2018-05-07

### Fixed

-   `Time` default display format changed to `00:00,00`.

## \[0.1.7\] - 2018-05-07

### Fixed

-   `Time::from()` works with `&str`s properly.
-   Removed a rogue print statement.

## \[0.1.6\] - 2018-05-07

### Added

-   Added `.from_bytes()` for `Level`, `Replay` and `State` structs.

## \[0.1.5\] - 2018-05-07

### Fixed

-   No longer necessary to have mutable `Level` in order to use `.save()` method.

### Breaking

-   Removed pub `raw` field from `Level` struct.

## \[0.1.4\] - 2018-05-07

### Added

-   Added preliminary state.dat read/write support for best times.
-   Added `Time` struct with methods for converting string to `i32` represented time, and vice versa.

### Breaking

-   Changed `.get_raw()` to `.to_bytes()` on Level struct.

## \[0.1.3\] - 2016-12-28

### Added

-   Added very basic topology checking.

### Fixed

-   All missing documentation.
-   Rewrote error handling slightly.
-   Refactored cargo structure.

## \[0.1.2\] - 2016-06-23

### Fixed

-   Remove slice pattern feature, and use str for matching instead.

## \[0.1.1\] - 2016-06-22

### Fixed

-   Match against slice to fix error with nightly.

## \[0.1.0\] - 2016-06-22

### Added

-   First release.
