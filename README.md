# swc-plugin-add-import-extension

> [`SWC`](https://swc.rs) plugin to add extensions to [`esm`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Modules) import and export module names

## 📄 Notes

Need to detect:
- local imports/exports (not possible w/o node resolver)
- alias imports/exports (shall we read ts paths?/perhaps better to provide regex in plugin cfg)
- *baseUrl imports/exports (not possible w/o node resolver + swc still does not support this atm)

## 🎉 Contributing

Contributions are welcome! Whether it is a small documentation change or a breaking feature, we welcome it!

_Please note: All contributions are taken under the MIT license_

## 👥 Contributors

<!-- readme: contributors -start -->
<table>
<tr>
    <td align="center">
        <a href="https://github.com/medfreeman">
            <img src="https://avatars.githubusercontent.com/u/1805267?v=4" width="50;" alt="medfreeman"/>
            <br />
            <sub><b>medfreeman</b></sub>
        </a>
    </td></tr>
</table>
<!-- readme: contributors -end -->
