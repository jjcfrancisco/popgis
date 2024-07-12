# popgis
A blazing fast way to insert large GeoJSON & ShapeFile into a PostGIS database.

## Why?

Loading large datasets into a PostGIS database can take a long time and reducing the completion time of such jobs time was the main aim of this project. `popgis` can be **x2 faster than ogr2ogr** and it's most noticeable when the input file is very large (with small dataset the performance increase is not as obvious) but also when working against non-local databases.

## Installation
You can install `popgis` via `Cargo` or `Homebrew`. Choose one option from below:

### Cargo
```bash
cargo install popgis
```

### Homebrew
```bash
brew tap jjcfrancisco/popgis
brew install popgis
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
* <del>Allow GeoJSON as input.</del>

## License
See [`LICENSE`](./LICENSE)
