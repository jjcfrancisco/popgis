@try-shapefile:
    cargo build --release
    cd ./target/release/ && ./popgis --input ../../examples/shapefile/andalucia.shp \
                 --uri  postgresql://pio:password@localhost:25432/popgis \
                 --schema shapefile \
                 --table andalucia

@try-shapefile-to-webmercator:
    cargo build --release
    cd ./target/release/ && ./popgis --input ../../examples/shapefile/andalucia.shp \
                 --uri  postgresql://pio:password@localhost:25432/popgis \
                 --schema shapefile \
                 --table andalucia3857 \
                 --srid=4326 \
                 --reproject=3857

@try-geojson:
    cargo build --release
    cd ./target/release/ && ./popgis --input ../../examples/geojson/spain.geojson \
                 --uri  postgresql://pio:password@localhost:25432/popgis \
                 --schema geojson \
                 --table spain

@try-geojson-to-webmercator:
    cargo build --release
    cd ./target/release/ && ./popgis --input ../../examples/geojson/spain.geojson \
                 --uri  postgresql://pio:password@localhost:25432/popgis \
                 --schema geojson \
                 --table spain3857 \
                 --srid=4326 \
                 --reproject=3857

@try-osmpbf:
    cargo build --release
    cd ./target/release/ && ./popgis --input ../../examples/osmpbf/monaco-latest.osm.pbf \
                 --uri  postgresql://pio:password@localhost:25432/popgis \
                 --schema osmpbf \
                 --table monaco \

@set-tags:
