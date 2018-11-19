# setup_tablet
This is a simple program to run the proper xinput to set my tablet up for use.

Really a shell script would have worked as well, but anything I can use to learn rust better.

So I have a UGEE 2150 drawing tablet, and have to use xinput to set the mouse and pen 
to the HDMI-1 display. I'm to lazy to run the commands every start up so thats why this exist.

For refrence you need to run:
xrandr
xinput --list
xinput map-to-output id deivice 

plex@bearz ~ $ xrandr
Screen 0: minimum 320 x 200, current 3840 x 1080, maximum 16384 x 16384
DVI-I-1 connected primary 1920x1080+0+0 (normal left inverted right x axis y axis) 476mm x 268mm
   1920x1080     60.00*+
   1680x1050     59.88  
   1280x1024     75.02    60.02  
   1440x900      59.90  
   1280x960      60.00  
   1280x800      59.91  
   1152x864      75.00  
   1280x720      60.00  
   1024x768      75.03    70.07    60.00  
   832x624       74.55  
   800x600       72.19    75.00    60.32    56.25  
   640x480       75.00    66.67    59.94  
   720x400       70.08  
DP-1 disconnected (normal left inverted right x axis y axis)
DP-2 disconnected (normal left inverted right x axis y axis)
DP-3 disconnected (normal left inverted right x axis y axis)
HDMI-1 connected 1920x1080+1920+0 (normal left inverted right x axis y axis) 575mm x 323mm
   1920x1080     60.00*+  60.00    50.00    59.94  
   1920x1080i    60.00    50.00    59.94  
   1680x1050     59.88  
   1280x1024     60.02  
   1280x960      60.00  
   1366x768      59.96  
   1280x800      59.91  
   1280x720      60.00    50.00    59.94  
   1024x768      75.03    70.07    60.00  
   800x600       72.19    75.00    60.32    56.25  
   720x576       50.00  
   720x480       60.00    59.94  
   640x480       75.00    72.81    60.00    59.94  
   720x400       70.08  

plex@bearz ~ $ xinput --list
⎡ Virtual core pointer                    	id=2	[master pointer  (3)]
⎜   ↳ Virtual core XTEST pointer              	id=4	[slave  pointer  (2)]
⎜   ↳ USB Optical Mouse                       	id=8	[slave  pointer  (2)]
⎜   ↳ HP USB Multimedia Keyboard Consumer Control	id=10	[slave  pointer  (2)]
⎜   ↳ UGEE TABLET MONITOR Mouse               	id=14	[slave  pointer  (2)]
⎜   ↳ UGEE TABLET MONITOR Pen Pen (0)         	id=16	[slave  pointer  (2)]
⎣ Virtual core keyboard                   	id=3	[master keyboard (2)]
    ↳ Virtual core XTEST keyboard             	id=5	[slave  keyboard (3)]
    ↳ Power Button                            	id=6	[slave  keyboard (3)]
    ↳ Power Button                            	id=7	[slave  keyboard (3)]
    ↳ HP USB Multimedia Keyboard              	id=9	[slave  keyboard (3)]
    ↳ HP USB Multimedia Keyboard System Control	id=11	[slave  keyboard (3)]
    ↳ HP USB Multimedia Keyboard              	id=12	[slave  keyboard (3)]
    ↳ UGEE TABLET MONITOR Pen                 	id=13	[slave  keyboard (3)]
    ↳ HP USB Multimedia Keyboard Consumer Control	id=15	[slave  keyboard (3)]
plex@bearz ~ $ xinput map-to-output 14 HDMI-1 
plex@bearz ~ $ xinput map-to-output 16 HDMI-1 

