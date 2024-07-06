@try:
    cargo run -- --input ./examples/shapefile/small-example.shp \
                 --uri  postgresql://pio:password@localhost:25432/popgis \
                 --table waters
