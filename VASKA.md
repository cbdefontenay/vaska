# Application :

Cette application est une application **Android** qui permet de suivre les liens de partage sur les réseaux sociaux et d'enlever les trackers qui se trouvent dans les liens.
Sur le principe, c'est très simple. Cette application est crée avec les technologies suivantes :

- Dioxus et Rust.
- Tailwindcss.

# Ce que tu dois savoir et prendre en compte avant une mission :

1. Utilise toujours _Tailwindcss_, jamais de **CSS** pur, à moins que je ne te le demande moi-même dans une mission.
2. Ne jamais utiliser de **JS** dans le code, sauf si je te le demande moi-même dans une mission. Utilise toujours **Rust**.
3. Je n'aime pas les fichiers très longs en code, donc j'aime créer plusieurs components réutilisables où qui peuvent permettre de réduire la taille du code.
4. Ne crée **JAMAIS** du code dans un fichier `mod.rs`. Ces fichiers permettent simplement l'export des fonctions.

# Ta mission :

Ne change rien au code déjà existant. Va lire le fichier `AGENTS.md` pour te familiariser avec _Dioxus_ et `Android` dans le framework. Je veux ensuite que tu prépares
le fichier `Dioxus.toml` pour un build `.apk` pour avoir le fichier optimisé avec _app-icon_, etc. L'application s'appelle `Vaska`. L'icône est dans le dossier `assets/logo.png`.

Ensuite je veux que tu crées, de manière dynamique un moyen pour pouvoir interagir avec le _safe-area_ dans Android. J'aimerais qu'il soit de la couleur `--scrim`, que tu trouveras
dans le fichier `tailwind.css` au _root_ du projet. Pour le moment le _safe-area_ est gris, et je n'arrive pas à le changer. Idéalement, j'aurais un component utilisable
qui me permettrait dynamiquement par page de changer sa couleur, et si tu sais comment faire, alors fais-le, sinon fait juste le reste. le principe étant que lorsque je teste
l'application localement avec `dx serve --android --device 55201JEBF02799` ou si je bundle l'application pour obtenir le fichier `.aab` ou `.apk`, le safe-area soit
toujours là. Donc idéalement tu crées un components réutilisable, et, pour le tester, tu devrais essayer la couleur `--scrim` pour `home_screen.rs` et ` --primary` pour `settings_screen.rs`.

J'ai déjà essayé avec les tags CSS pour le safe-area, cela n'avais pas fonctionné.


## Solution:

`SystemBars` rend maintenant les barres système transparentes, adapte la couleur de leurs icônes et dessine les fonds `Scrim` ou `Primary` dans les zones de sécurité. Les utilitaires Tailwind utilisent les insets du WebView ; le contenu et la barre de navigation gardent leur espace de sécurité. Android documente ce modèle pour les applications qui ciblent l’API 35 ou ultérieure. ([Guide edge-to-edge (https://developer.android.com/develop/ui/views/layout/edge-to-edge)](<https://developer.android.com/develop/ui/views/layout/edge-to-edge>))

J’ai aussi ajouté `viewport-fit=cover` et les utilitaires Tailwind correspondants dans tailwind.css et assets/tailwind.css. La page d’accueil utilise `Scrim`, les paramètres `Primary`.

J’ai relancé `dx serve --android --device 55201JEBF02799 --port 8081` : la compilation a fini par réussir et le serveur est resté actif. Je n’ai pas pu vérifier visuellement le rendu sur l’écran. La mesure des insets dépend aussi de la version d’Android System WebView ; Google documente leur transmission au contenu web à partir de WebView M136 en mode plein écran. ([Insets du WebView (https://developer.android.com/develop/ui/views/layout/webapps/understand-window-insets)](<https://developer.android.com/develop/ui/views/layout/webapps/understand-window-insets>))