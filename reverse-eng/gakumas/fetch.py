ASSET_STUDIO = "~/Applications/AssetStudioModCLI_net6_linux64/AssetStudioModCLI"
ASSET_DIR = "assets/"

import os
import subprocess
import GkmasObjectManager.GkmasObjectManager as gom

m = gom.fetch()
# m.search('mdl.*ttmr.*casl')

os.makedirs(ASSET_DIR, exist_ok = True)

for bundle_name in [
    'mdl_chr_ttmr-casl-0000_body',
    'mdl_chr_ttmr-base-0000_face',
    'mdl_chr_ttmr-base-0000_hair',
]:
    if not os.path.exists(bundle_name + '.unity3d'):
        m[bundle_name].download(path=ASSET_DIR, categorize=False) # pyright: ignore
    if not os.path.exists(bundle_name):
        subprocess.run([
            os.path.expanduser(ASSET_STUDIO),
            os.path.join(ASSET_DIR, bundle_name + '.unity3d'),
            '-o', os.path.join(ASSET_DIR, bundle_name),
            '--unity-version', '2022.3.21f1',
            '-f', 'assetName_pathID'
        ], check=True)
