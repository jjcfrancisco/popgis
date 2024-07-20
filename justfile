@try-shapefile:
    cargo build --release
    cd ./target/release/ && ./popgis --input ./examples/shapefile/andalucia.shp \
                 --uri  postgresql://pio:password@localhost:25432/popgis \
                 --schema shapefile \
                 --table andalucia

@try-geojson:
    cargo build --release
    cd ./target/release/ && ./popgis --input ../../examples/geojson/spain.geojson \
                 --uri  postgresql://pio:password@localhost:25432/popgis \
                 --schema geojson \
                 --table spain

@set-tags:
