# popgis
A blazing fast way to insert Shapefiles & GeoJSON into a PostGIS database.

## Install
*Upcoming...*

## Usage
Required flags are `input`, `uri` & `table`.

1. Use the **input** flag to choose the *shapefile* or *geojson* file to insert into a PostGIS database.
2. Use the **uri** flag to tell `popgis` the PostGIS database where you'd like to insert the input data.
3. The **schema** you would like to insert the table. This is optional.
4. The **table** flag is for choosing the name of the resulting table. If no `schema` is provided, the table is created in the *public* schema.

## Examples
```bash
popgis --input water_polygons.shp \
       --uri  postgresql://my_username:my_password@localhost:5432/my_database \
       --schema osm \
       --table waters
```

## License
See [`LICENSE`](./LICENSE)

## Benchmarks
*Upcoming...*

## Limitations
Currently, only ShapeFile files are implemented.

## Future implementation
The list below contains the upcoming implementations.

To do:

* Allow GeoJSON as input.
