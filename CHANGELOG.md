# Change Log

All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

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
