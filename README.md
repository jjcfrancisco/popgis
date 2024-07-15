# PopGIS
A blazing fast way to insert large GeoJSON & ShapeFile into a PostGIS database.

## Why?
Importing large datasets into a PostGIS database can take a long time and the aim of Popgis is to optimize the performance of such operations. **Popgis can be up to twice as fast as ogr2ogr**, particularly with very large input files against remote databases. Although the performance improvement for smaller datasets may be minimal, the efficiency gains for larger datasets are considerable. For more details, go to the [benchmarks](#benchmarks) section.

## Installation

### Binary
For binary distributions see the table below

| Platform | x64 | ARM-64 |
|----------|-----|--------|
| Linux    |     |        |
| macOS    |     |        |
| Windows  |     |        |

### Cargo
If you prefer using Cargo, you can install Popgis directly by running the Cargo install command
```bash
cargo install popgis
```

### Homebrew
For macOS users with Homebrew, you can install Popgis by adding the appropriate Homebrew tap
```bash
brew tap jjcfrancisco/popgis
brew install popgis
popgis --help
```

## Usage
`input`, `uri` & `table` are required; `schema` & `srid` are optional.

**Flags**

`input`: choose the *geojson* or *shapefile* file to insert into a PostGIS database.

`uri`: the PostGIS database where you'd like to insert the input data.

`schema`: where you would like the specified table. **Optional**. *Default is public.*

`table`: choose the name of the resulting table.

`srid`: choose either 4326 (WGS84) or 3857 (Web Mercator).  **Optional**. *Default is 4326.*

**Examples**
```bash
## GeoJSON -> PostGIS ##
popgis -i spain.geojson \
       -u postgresql://my_username:my_password@localhost:5432/my_database \
       -s osm \
       -t waters \
       --srid 3857

## ShapeFile -> PostGIS ##
popgis -i water_polygons.shp \
       -u  postgresql://my_username:my_password@localhost:5432/my_database \
       -s osm \
       -t waters
```

## Benchmarks

### ShapeFile

| file size |  `popgis` took | `ogr2ogr` took | environment |
|-----------|----------------|----------------|-------------|
| 1.2GB     | **36sec**      | 1min 15sec     | local [PostGIS](https://hub.docker.com/r/kartoza/postgis/)       | 

> The file used for this test can be found [here](https://osmdata.openstreetmap.de/data/water-polygons.html).

### GeoJSON

| file size |  `popgis` took | `ogr2ogr` took | environment |
|-----------|----------------|----------------|-------------|
| 103.9MB   | **2sec**       | 5sec           | local [PostGIS](https://hub.docker.com/r/kartoza/postgis/)       | 

> The file used for this test can be found [here](https://data.cityofnewyork.us/City-Government/NYC-Street-Centerline-CSCL-/exjm-f27b).

## Future implementation
The list below contains the upcoming implementations.

To do:

* Allow nested GeoJSON properties.
* Improve stdout.
* Modes: create, overwrite & append.
* <del>Allow GeoJSON as input.</del>

## License
See [`LICENSE`](./LICENSE)
