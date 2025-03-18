# tympanon/alexa_sdk

This is an experimental fork of [alexa_rust](https://github.com/arienmalec/alexa_rust) with a few goals:
- fix runtime incompatibilities with Alexa's behavior in 2025
- add support for some important missing response fields, notably Directives
- establish mechanisms for extensibility in cases where there are missing parts of the data model
- normalize all naming conventions so that it becomes possible to programmatically generate the rust data types, either from example JSON or from a supported SDK in a different language.

See the [parent repository](https://github.com/arienmalec/alexa_rust) for more general / official information about this crate.
