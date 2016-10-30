# Changelog

This project follows semantic versioning.

### Unpublished
    - [added] Functions to the `Pow` and `Len` traits. This is *technically* a breaking change, but
      it would only break someone's code if they have a custom impl for `Pow`. I would be very
      surprised if that is anyone other than me.
    - [added]

### 1.4.0 (2016-10-29)
    - [added] Type-level arrays of type-level integers. (PR #66)
    - [added] The types in this crate are now instantiable. (Issue #67, PR #68)

### 1.3.1 (2016-03-31)
    - [fixed] Bug with recent nightlies.

### 1.3.0 (2016-02-07)
    - [changed] Removed dependency on libstd. (Issue #53, PR #55)
    - [changed] Reorganized module structure. (PR #57)


### 1.2.0 (2016-01-03)
    - [added] This change log!
    - [added] Convenience type aliases for operators. (Issue #48, PR #50)
    - [added] Types in this crate now derive all possible traits. (Issue #42, PR #51)
