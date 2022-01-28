// Recommodations from http://wiki.webmproject.org/ffmpeg/vp9-encoding-guide
ffmpeg -i <source> -c:v libvpx-vp9 -pass 1 -b:v 1000K -threads 8 -speed 4
-tile-columns 6 -frame-parallel 1
-an -f webm /dev/null


ffmpeg -i <source> -c:v libvpx-vp9 -pass 2 -b:v 1000K -threads 8 -speed 1
-tile-columns 6 -frame-parallel 1 -auto-alt-ref 1 -lag-in-frames 25
-c:a libopus -b:a 64k -f webm out.webm
