

TODO
=====

Export of the grocery and the planning to a pdf file.

Export of the grocery and the planning to a mardown file.

Data model for planning and grocery list history

Data model for recurring item purchase.

Data compution to sum ingredient from planning into grocery list.

Export of the grocery and the planning to a mardown file.

Data model for grocery list.

Data model for planning with days, lunchs.

GUI edition of recipe: renaming of recipe. Update name in the left pane
recipes list.

GUI grocery section deletion.
GUI grocery section renaming.
GUI grocery section creation.

GUI food group deletion.
GUI food group renaming.
GUI food group creation.

GUI ingredient edit grocery section
GUI ingredient edit food group
GUI ingredient edit default quantity

GUI ingredient deletion
GUI ingredient renaming
GUI ingredient creation

GUI deletion of recipe.

GUI creation of a new recipe.


DONE
=====

GUI edition of recipe: renaming of recipe.

GUI edition of recipe: ingredient removing.

GUI edition of recipe: ingredient quantity value and unit edition.

GUI edition of recipe: ingredient adding (with listing of all ingredient)

GUI edition of recipe: edition of intruction content.

GUI view of one recipe with name, intruction, ingredients list.

GUI view of the recipe list.

Data file save/load of the ingredient catalog (with section and group).

Data file save/load of the recipe book.

Data model for a list of recipe.

Data model for cooking recipe.

Data model for catalog of ingredient with list of shop section and food group.

Data model for ingredient.

Data model for ingredient quantity with unit.

Data model for shop sections.

Data model for food group.


-------------------------------------


doit gérer (créer/modifier/suppr) des recettes de cuisines, des listes de course, des planning, historique de planning, liste d'achat récurent



Aspect technique
==================

Logiciel mutli-lingue.

L'import/export des ficheirs doit être robuste aux différentes versions du logiciel.


Mode
=========

modes :

* gestion/rédaction d'un nouveau planning
    * acces à la liste des ingrédient
    * acces à la liste de recettes
* visualisation des recettes
    * possibilité de l'affecter à un planning rapidement
* gestion/rédaction d'une recette
    * acces à la liste des ingrédient
* gestion des listes de courses
    * vue de la sommation provenant du planning
    * vue de l'instance provenant de la liste récursive
    * acces à la liste des articles
    * modification des articles




Catalogue d'ingrédient et d'articles
======================================

Dans le catalogue capacité à mettre certain ingrédient en avant.

Un ingrédient est défini par

* un nom
* un conditionnement (conserve/bocal, pot, vrac, paquet) 
* un rayon où l'on le trouve en magasin
* une liste de magasin par ordre de préférence (ou l'on peut l'acheter)
* une unité par défaut (g, cc, cs, ml, l, kg, unité/portion)
* une quantité par défaut
* groupe alimentaire (sucré, légume, fromage, fruit, protéine, féculent, lipides, laitage)



Recette
=====================

Dans le catalogue capacité à mettre certain ingrédient en avant.

import/export recettes de cuisines

recettes est un ensemble d'ingrédient, des instructions et des illustrations

un ingrédient peut être une autre recette

les instructions de recettes sont un texte mise en forme



liste de courses
=================================
LC est en ensemble d'article qui proviennent soit

* des menus d'un planning
* de la liste d'achat récurent

Une liste de course en une période de temps (avec une date de début et de fin).

Les créations des articles à acheter depuis la liste d'achat récurent est soumis à une récurrence d'achat.

Doit pouvoir exporter, c'est à dire générer un document autonome permet de faire les courses.





menu / planning
==================

On doit visualiser des creneaux de jour et pouvoir mettre des ingrédients ou des
recettes dedans.

Certains ingrédient ne seront pas systétiquement mis en list de courses (car stocké
en grande quatité par rapport à l'usage typique).

Un planning est un ensemble de journée.

Un journée est une date et un jour de la semain et possède des repas/créneau.

Un repas possède un nom (petit dej, etc) et un menu.

Un menu est un choix de recette et/ou des choix ingrédient.

On doit pouvoir des créneaux/repas (ajouter, enlever).

Un repas/crean possède un nom.

Pour une journée donnée, il a des limites min et max sur ce qu'on peut manger.
Par repas/créneau et selon le type de repas/creneau il y a des règles indicative à respecter.
Exemple :

* Pas plus de 3 portion de fruits par jour.
* Pas plus de 3 portions de matière grasse par jour.
* Pas trop souvent d'ingrédient sur une période glissante

Gérer un historique des anciens menus. On doit pouvoir rechercher sur des date et/ou créneau.
On doit pouvoir récupérer des anciens menu dans le planning en cours.

On doit pouvoir transformer un menu (en partie ou totalement) en recette. Soit pour en créer
un nouveau ou alors mettre à jour une existante.


