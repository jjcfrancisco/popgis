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

`input`: choose the *geojson* or *shapefile* file to insert into a PostGIS database.

`uri`: the PostGIS database where you'd like to insert the input data.

`schema`: where you would like the specified table. **This is optional**. *Default is public.*

`table`: choose the name of the resulting table.

**Examples**
```bash
## GeoJSON -> PostGIS ##
popgis --input spain.geojson \
       --uri  postgresql://my_username:my_password@localhost:5432/my_database \
       --schema osm \
       --table waters

## ShapeFile -> PostGIS ##
popgis --input water_polygons.shp \
       --uri  postgresql://my_username:my_password@localhost:5432/my_database \
       --schema osm \
       --table waters
```

## Benchmarks
*Upcoming...*

## Future implementation
The list below contains the upcoming implementations.

To do:

* <del>Allow GeoJSON as input.</del>

## License
See [`LICENSE`](./LICENSE)