# swc-plugin-add-import-extension

> SWC plugin to add extensions to esm import and export specifiers

## Notes

Need to detect:
- local imports/exports (not possible w/o node resolver)
- alias imports/exports (shall we read ts paths?/perhaps better to provide regex in plugin cfg)
- *baseUrl imports/exports (not possible w/o node resolver + swc still does not support this atm)