#!/usr/local/bin/amm --no-remote-logging

import $ivy.`net.debasishg:redisclient_2.13:3.30`

import com.redis._
import java.io._
import sys.process._

val r = new RedisClient("127.0.0.1", 6379)
val hd = System.getProperty("user.dir")
val wd = hd + "/schema/"
val listfiles = (s"ls $wd").!!
listfiles.split('\n').foreach(fn => if (fn.length() > 1) (s"rm -f $wd$fn").!)

val fn = r.get("schemaImgFileName").getOrElse("schemaImgO")
val wdfn = wd + fn
var out = None: Option[FileOutputStream]
out = Some(new FileOutputStream(wd + fn + "_1.jpg"))

out.get.write(java.util.Base64.getDecoder.decode(r.get("schemaImg").getOrElse(",").split(',')(1)))

if (out.isDefined) out.get.close

val u="_"
val c = "1"

val ares = r.get("schemaImgAspectResolution").getOrElse("1:1_640x640,1280x1280,1920x1920;4:3_640x480,1280x960,1920x1440;16:9_640x360,854x480,1280x720,1920x1080")

println(ares)

ares.split(";").foreach( ar => {
  val item_ar = ar.split('_')
  val aa = item_ar(0).split(':')
  val a_w = aa(0)
  val a_h = aa(1)

  item_ar(1).split(',').foreach( res => {
    val item_res = res.split('x')
    val res_w = item_res(0)
    val res_h = item_res(1)

    println(a_w, a_h, res_w, res_h)
  })
})

for ((a, b, x, s)  <- List(
/*    ("16", "9", "x", List(("640", "360"), ("854", "480"), ("1280", "720"), ("1920", "1080")))
	, ("4", "3", "x", List(("640", "480"), ("1280", "960"), ("1920", "1440")))
	, ("1", "1", " ", List(("640", "640"), ("1280", "1280"), ("1920", "1920")))
	,*/ ("o","o", " ", List(("320", "320"),("640", "640"), ("1280", "1280"), ("1920", "1920"))))) {
		if (a != "o") {
			(s"$hd/scripts/aspectcrop -a $a:$b -g c $wdfn$u$c.jpg $wdfn$u$a$u$b$u$c.jpg").!
		}
		for( (ins) <- s) {
			val sx = ins._1
			val sy = ins._2
			if (a != "o") {
				(s"convert $wdfn$u$a$u$b$u$c.jpg -resize '$x$sy' $wdfn$u$a$u$b$u$sx$u$c.jpg").!
			} else {
				(s"convert $wdfn$u$c.jpg -resize '$x$sy' $wdfn$u$a$u$sx$u$c.jpg").!
			}
		}
}


val imgWidth=(s"identify -format '%[fx:w]' $wdfn$u$c.jpg").!!
val imgHeight=(s"identify -format '%[fx:h]' $wdfn$u$c.jpg").!!

r.set(s"schemaImg$c"+"Width", imgWidth.stripLineEnd)
r.set(s"schemaImg$c"+"Height", imgHeight.stripLineEnd)

val fntgz = "schema.tgz"
val tmpfn = s"/tmp/$fntgz"
(s"tar -czf $tmpfn -C $wd .").!
(s"mv $tmpfn $wd").!

val lnfn = "schemaImg"
(s"ln -s $wdfn$u$c.jpg $wd$lnfn$u$c.jpg").!

("sleep 5").!

r.del("schemaImg")
