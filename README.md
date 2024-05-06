# DS210FinalProject


Overview: 

Originally I worked with a data set on chronic disease indicators https://www.kaggle.com/datasets/cdc/chronic-disease because I wanted to track the relationships between different chronic diseases and their symptoms. However, after cleaning and preparing the dataset I created the graph but the relationships between nodes and edges were not compatible with the tests I wanted to run. I decided to switch to a data set on the prevalence of different vaccine preventable diseases https://www.kaggle.com/datasets/rishidamarla/vaccine-preventable-diseases  based on county and year.  

I tested six degrees of separation between Diphtheria and Alameda which was 0. This makes sense since Diphtheria was a preventable disease that occurred in Alameda, if this wasn't the case then the degree of separation would be a non 0 integer. The average distance between pairs of vertices is 2.25. The diseases and counties both appear as nodes in the graph and the edges represent the prevalence of diseases within the connected county. For future use graphs like this can be used to track the spread of a preventable disease overtime and which ones pose the highest risk of being contractible


“dot -Tpng graph.dot -o graph.png” in the terminal creates a visual representation of the graph

On my laptop I used the path "/Users/kw/DS210FinalProject/prevDis.csv" , I have the csv file attached if it does not run right away. Additionally I did not use separate branches but there are 14 commits. Results are at the bottom of the pdf

