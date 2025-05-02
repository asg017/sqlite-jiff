.PHONY: timezone earthquakes all clean
all: timezones earthquakes

clean:
	rm -f timezones-with-oceans.geojson.zip timezones.geojson earthquakes.geojson

##### TIMEZONES #####
timezones-with-oceans.geojson.zip:
	curl -o $@ -L 'https://github.com/evansiroky/timezone-boundary-builder/releases/download/2025b/timezones-with-oceans.geojson.zip'

timezones.geojson: timezones-with-oceans.geojson.zip
	unzip -p $< combined-with-oceans.json > $@

timezones: timezones.geojson

##### EARTHQUAKES #####
earthquakes.geojson:
	curl -o $@ -L 'https://earthquake.usgs.gov/fdsnws/event/1/query.geojson?starttime=2010-01-01%2000:00:00&minmagnitude=6&orderby=time'

earthquakes: earthquakes.geojson




