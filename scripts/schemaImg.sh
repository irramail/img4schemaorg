#!/bin/sh
cd ~/schema
rm -f *.jpg
rm -f schema.tgz
rm -f schemaImg.b64
COUNTER=0
FILENAME=`redis-cli get schemaImgFileName`
redis-cli get schemaImg | sed "s/|/\n/g" | sed "s/^.*,//" >  ~/schema/schemaImg.b64
for  i in `cat  ~/schema/schemaImg.b64`;
do
  COUNTER=$((COUNTER + 1))
  echo $i | base64 -d > "$FILENAME"_"$COUNTER".jpg;
  
  /home/p6/scripts/aspectcrop -a 16:9 -g c "$FILENAME"_"$COUNTER".jpg "$FILENAME"_16_9_"$COUNTER".jpg
  /home/p6/scripts/aspectcrop -a 4:3 -g c "$FILENAME"_"$COUNTER".jpg "$FILENAME"_4_3_"$COUNTER".jpg
  /home/p6/scripts/aspectcrop -a 1:1 -g c "$FILENAME"_"$COUNTER".jpg "$FILENAME"_1_1_"$COUNTER".jpg

  convert "$FILENAME"_"$COUNTER".jpg -resize '640' "$FILENAME"_o_640_"$COUNTER".jpg
  convert "$FILENAME"_"$COUNTER".jpg -resize '1280' "$FILENAME"_o_1280_"$COUNTER".jpg
  convert "$FILENAME"_"$COUNTER".jpg -resize '1920' "$FILENAME"_o_1920_"$COUNTER".jpg

  convert "$FILENAME"_16_9_"$COUNTER".jpg -resize 'x360' "$FILENAME"_16_9_640_"$COUNTER".jpg
  convert "$FILENAME"_16_9_"$COUNTER".jpg -resize 'x480' "$FILENAME"_16_9_854_"$COUNTER".jpg
  convert "$FILENAME"_16_9_"$COUNTER".jpg -resize 'x720' "$FILENAME"_16_9_1280_"$COUNTER".jpg
  convert "$FILENAME"_16_9_"$COUNTER".jpg -resize 'x1080' "$FILENAME"_16_9_1920_"$COUNTER".jpg

  convert "$FILENAME"_4_3_"$COUNTER".jpg -resize 'x480' "$FILENAME"_4_3_640_"$COUNTER".jpg
  convert "$FILENAME"_4_3_"$COUNTER".jpg -resize 'x960' "$FILENAME"_4_3_1280_"$COUNTER".jpg
  convert "$FILENAME"_4_3_"$COUNTER".jpg -resize 'x1440' "$FILENAME"_4_3_1920_"$COUNTER".jpg

  convert "$FILENAME"_1_1_"$COUNTER".jpg -resize '640' "$FILENAME"_1_1_640_"$COUNTER".jpg
  convert "$FILENAME"_1_1_"$COUNTER".jpg -resize '1280' "$FILENAME"_1_1_1280_"$COUNTER".jpg
  convert "$FILENAME"_1_1_"$COUNTER".jpg -resize '1920' "$FILENAME"_1_1_1920_"$COUNTER".jpg

  tar -czf schema.tgz *.jpg
  ln -s /home/p6/schema/"$FILENAME"_"$COUNTER".jpg schemaImg_"$COUNTER".jpg
  redis-cli set schemaImg"$COUNTER"Width `identify -format "%[fx:w]" "$FILENAME"_"$COUNTER".jpg`
  redis-cli set schemaImg"$COUNTER"Height `identify -format "%[fx:h]" "$FILENAME"_"$COUNTER".jpg`
done

sleep 5
redis-cli del schemaImg
