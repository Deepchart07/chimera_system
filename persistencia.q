/ Definicion de la tabla de trades
trades:([]time:`timespan$();sym:`symbol$();price:`float$();size:`float$());

/ Funcion para guardar datos y limpiar memoria
save_every_4h:{
    path:.Q.par[`:data/history;.z.d;`trades];
    path insert select from trades;
    delete from `trades;
    -1 "Respaldo de trades completado a las ",string .z.p;
 };
