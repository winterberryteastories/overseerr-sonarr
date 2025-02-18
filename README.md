The goal of `overseerr-sonarr` is to allow automatic setting of 1) the root folder 2) the quality profile, and 3) series type based on certain criteria.

Overseerr allows selecting one quality profile for standard and one for anime. Additionally all anime is added as series type "Anime / Absolute" without choice.

To make this work properly to the following settings in overseerr:

* Apply `tv` and `overseerr-queue` tags for standard shows.
* Apply `anime` and `overseerr-queue` tags for anime shows.
* Disable `Enable Automatic Search`

Currently the behavior is:
* All anime is set to series type "Standard".
* German shows get a different root folder (`/data/media/tv-de`).
* German shows get a different quality profile.

# TODO
Currently this is not really customizable but if someone shows interest I'd be happy to improve it :)
