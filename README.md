# popgis
A blazing fast way to insert GeoJSON & ShapeFile into a PostGIS database.

## Installation
```bash
brew tap jjcfrancisco/popgis
brew install popgis
```

## Usage
`input`, `uri` & `table` are required; `schema` is optional.

**Flags**

`input`: choose the *shapefile* or *geojson* file to insert into a PostGIS database.

`uri`: the PostGIS database where you'd like to insert the input data.

`schema`: where you would like to insert the table. *This is optional.*

`table`: choose the name of the resulting table. *Default is public*.

**Example**
```bash
popgis --input water_polygons.shp \
       --uri  postgresql://my_username:my_password@localhost:5432/my_database \
       --schema osm \
       --table waters
```

## Benchmarks
*Upcoming...*

## Limitations
Currently, only ShapeFile files are implemented.

## Future implementation
The list below contains the upcoming implementations.

To do:

* Allow GeoJSON as input.

## License
See [`LICENSE`](./LICENSE)