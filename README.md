# PopGIS
A blazing fast way to insert large GeoJSON & ShapeFile into a PostGIS database.

## Why?
Importing large datasets into a PostGIS database can take a long time and the aim of Popgis is to optimize the performance of such operations. **Popgis is 2x faster than ogr2ogr**, particularly with very large input files against remote databases (see [benchmarks](#benchmarks)). Although the performance improvement for smaller datasets may be minimal, the efficiency gains for larger datasets are considerable. For more details, go to the [benchmarks](#benchmarks) section.

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

`input` (short: `-i`): choose the *geojson* or *shapefile* file to insert into a PostGIS database.

`uri` (short: `-u`): the PostGIS database where you'd like to insert the input data.

`schema` (short: `-s`): where you would like the specified table. **Optional**. *Default is public.*

`table` (short: `-t`): choose the name of the resulting table.

`srid`: choose either 4326 (WGS84) or 3857 (Web Mercator).  **Optional**. *Default is 4326.*

**Examples**
```bash
## GeoJSON -> PostGIS ##
popgis --input spain.geojson \
       --uri postgresql://my_username:my_password@localhost:5432/my_database \
       --schema osm \
       --table waters \
       --srid 3857

## ShapeFile -> PostGIS ##
popgis -i water_polygons.shp \
       -u  postgresql://my_username:my_password@localhost:5432/my_database \
       -s osm \
       -t waters
```

## Benchmarks
Although non extensive, the benchmarking shows **Popgis is twice faster than ogr2ogr**. This is most noticeable with large files.

### ShapeFile

| file size |  `popgis` took | `ogr2ogr` took | environment |
|-----------|----------------|----------------|-------------|
| 1.2GB     | **36sec**      | 1min 15sec     | local [PostGIS](https://hub.docker.com/r/kartoza/postgis/)       | 
| 1.2GB     | **36min**      | 1h 14min       | virtual machine ([n2-standard-4](https://cloud.google.com/compute/docs/general-purpose-machines)) [PostGIS](https://hub.docker.com/r/kartoza/postgis/) |

> The file used for this test can be found [here](https://osmdata.openstreetmap.de/data/water-polygons.html).

### GeoJSON

| file size |  `popgis` took | `ogr2ogr` took | environment |
|-----------|----------------|----------------|-------------|
| 103.9MB   | **2sec**       | 5sec           | local [PostGIS](https://hub.docker.com/r/kartoza/postgis/)       | 
| 103.9MB   | **2min 14sec** | 5min           | virtual machine ([n2-standard-4](https://cloud.google.com/compute/docs/general-purpose-machines)) [PostGIS](https://hub.docker.com/r/kartoza/postgis/) |

> The file used for this test can be found [here](https://data.cityofnewyork.us/City-Government/NYC-Street-Centerline-CSCL-/exjm-f27b).

## Future implementation
The list below contains the upcoming implementations.

To do:

* Allow nested GeoJSON properties.
* <del>Improve stdout.</del>
* Modes: create, overwrite & append.
* <del>Allow GeoJSON as input.</del>

## License
See [`LICENSE`](./LICENSE)
