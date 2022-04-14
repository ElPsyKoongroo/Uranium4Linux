# Uranium4Linux
 
<h4> Minecraft Console Mod Downloader</h4>


Version 1.3.0 (Functional)


Avalible repositories: <br>
- RINTH

<h4> Commands </h4>

- page + 'page_number' 
- mod&nbsp; + 'mod_number' 
- make 
- exit

The program is capable of downloading mods, modpacks and make modpacks from a folder.

To download a modpack (generated by this program) compile as bin uranium_loader and execute with the '-d' flag 
as first argument, the modpack file as the second argument and the destination folder as third argument.

Example: <br>
<ol>
<li> cargo build -p uranium_loader --bin uranium_loader</li>
<li> uranium_loader.exe -d path/to/modpack/file destination/path/</li>
<li> Done !</li>
</ol>

To update a modpack (generated by this program) compile as bin uranium_loader and run it with '-u' as first argument and the path to the modpack as second argument.

Example: <br>
<ol>
<li> cargo build -p uranium_loader --bin uranium_loader</li>
<li> uranium_loader.exe -u path/to/modpack/file </li>
<li> DONE ! </li>
</ol>

We also have a Windows version with GUI, you can download [here]. <br> 





[here]: https://github.com/ElPsyKoongroo/MinecraftModDownloader
