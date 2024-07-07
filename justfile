@try:
    cargo run -- --input ./examples/shapefile/small-example.shp \
                 --uri  postgresql://pio:password@localhost:25432/popgis \
                 --schema test \
                 --table waters

@build-and-run:
    cargo build --release
    cd ./target/release/ && ./popgis --input ~/Downloads/water-polygons-split-4326/water_polygons.shp \
                 --uri  postgresql://pio:password@localhost:25432/popgis \
                 --schema osm \
                 --table waters
