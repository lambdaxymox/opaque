# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
Change log dates follow the ISO 8601 standard (YEAR-MONTH-DAY).

## [1.3.0] - 2025-07-25
- Add `get_key_value_mut` method to round out the ontology of accessor methods.
- Add `From` implementation for converting index map with unit typed values to index sets.

## [1.2.0] - 2025-07-16
- Use `alloc::Global` as the default parameter for the allocator type on ExtractIf iterators.

## [1.1.0] - 2025-07-16
- Add `Default` implementation for `Keys` iterator for index maps.

## [1.0.0] - 2025-07-15
- Initial release of library.
