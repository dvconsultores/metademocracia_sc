import { near, BigInt, JSONValue, json, ipfs, log, TypedMap, Value, typeConversion, BigDecimal, bigInt, bigDecimal } from "@graphprotocol/graph-ts"
import { Serie, Nft, Datanft, Owners } from "../generated/schema"

export function handleReceipt(receipt: near.ReceiptWithOutcome): void {
  const actions = receipt.receipt.actions;
  for (let i = 0; i < actions.length; i++) {
    handleAction(
      actions[i], 
      receipt.receipt, 
      receipt.outcome,
      receipt.block.header
    );
  }
}

//const list_contract_atributos_referencia = [];


function handleAction(
  action: near.ActionValue,
  receipt: near.ActionReceipt,
  outcome: near.ExecutionOutcome,
  blockHeader: near.BlockHeader
): void {
    
  if (action.kind != near.ActionKind.FUNCTION_CALL) return;
  
  // const data = action.toFunctionCall();

  //se obtiene el nombre del metodo que fue ejecutado en el smart contract
  const methodName = action.toFunctionCall().methodName;
  
  
  if (methodName == 'update_nft_series') {
    if(outcome.logs.length > 0) {
      const outcomeLog = outcome.logs[0].toString();
      
      if(!json.try_fromString(outcomeLog).isOk) return
      let outcomelogs = json.try_fromString(outcomeLog);
      const jsonObject = outcomelogs.value.toObject();
      
      if (jsonObject) {
        const logJson = jsonObject.get('params');
        if (!logJson) return;
  
        const data = logJson.toObject();
        
        const token_series_id = data.get('token_series_id')
        const metadatalog = data.get('token_metadata')
  
        if (!token_series_id || !metadatalog) return
  
        //convertimos la variable metadata en un objeto para poder acceder a sus variebles internas
        const metadata = metadatalog.toObject()
        
        //en caso de que no se transformable en un objeto se detiene la funcion
        if(!metadata) return

        //declaramos las variables dentro del objeto metadata
        const title = metadata.get('title')
        const description = metadata.get('description')
        const media = metadata.get('media')
        const price = data.get('price')
        const extra = metadata.get('extra')
        
        //se verifica que todas las variables que necesitamos existan en el objeto metadata
        if(!title || !description || !media || !price || !extra) return
        
        //if(title.isNull() || media.isNull()) return
        let serie = Serie.load(token_series_id.toString())
        if (serie) {
          if(!title.isNull()) { serie.title = title.toString() }
          if(!description.isNull()) { serie.description = description.toString() }
          if(!media.isNull()) { serie.media = media.toString() }
          if(!price.isNull()) { serie.price = BigDecimal.fromString(price.toString()) }
          if(!extra.isNull()) {
            serie.extra = extra.toString()
          }
          
          serie.save()
        }
        
      }
    }
  }


 //dfdsfsdfsdfsdf
 //asddasdsd
  if (methodName == 'nft_series') {
    if(outcome.logs.length > 0) {
      const outcomeLog = outcome.logs[0].toString();
      
      if(!json.try_fromString(outcomeLog).isOk) return
      let outcomelogs = json.try_fromString(outcomeLog);
      const jsonObject = outcomelogs.value.toObject();

      if (jsonObject) {
        const logJson = jsonObject.get('params');
        if (!logJson) return;
        const data = logJson.toObject();
        
        const serie_id = data.get('token_series_id')
        const creator_id = data.get('creator_id')
        const price = data.get('price')
        const metadatalog = data.get('token_metadata')
        

        if (!serie_id || !creator_id || !price || !metadatalog) return

        //convertimos la variable metadata en un objeto para poder acceder a sus variebles internas
        const metadata = metadatalog.toObject()

        //en caso de que no se transformable en un objeto se detiene la funcion
        if(!metadata) return

        //declaramos las variables dentro del objeto metadata
        const title = metadata.get('title')
        const description = metadata.get('description')
        const media = metadata.get('media')
        const extra = metadata.get('extra')
        const copies = metadata.get('copies')
        const reference = metadata.get('reference')

        //se verifica que todas las variables que necesitamos existan en el objeto metadata
        if(!title || !description || !media || !extra || !reference) return

        if(title.isNull() || media.isNull() || reference.isNull()) return
        
        //log.warning('paso {}', ["1.2"])
  
        let serie = Serie.load(serie_id.toString())

        if (!serie) {
          serie = new Serie(serie_id.toString())
          serie.title = title.toString()
          if(!description.isNull()) { serie.description = description.toString() }
          serie.media = media.toString()
          if(!extra.isNull()) { serie.extra = extra.toString() }
          serie.reference = reference.toString()
          serie.creator_id = creator_id.toString()
          if(!price.isNull()) { 
            serie.price = BigDecimal.fromString(price.toF64().toString())
            serie.price_near = bigDecimal.fromString("0")//BigDecimal.fromString(price.toString()).divDecimal(BigDecimal.fromString("1000000000000000000000000"))
          }
          serie.supply = BigInt.fromString("0")
          if(copies) { 
            if(!copies.isNull()) { serie.copies = copies.toBigInt() } 
          }
          serie.fecha = BigInt.fromU64(blockHeader.timestampNanosec)
          serie.save()
        }
        
      }
    }
    //let utcSeconds = (blockHeader.timestampNanosec / 1000000);
    //let date = new Date(utcSeconds)
    
    //log.warning("fehca: {} ----  fecha epoch: {}", [date.toISOString().split('T')[0].toString(), utcSeconds.toString()])
  }
  


  //este evento es disparado cuando el metodo es create_form
  if (methodName == 'nft_mint' || methodName == 'nft_buy' || methodName == 'nft_mint_for') {  
    if(outcome.logs.length > 0) {
      for (let index = 0; index < outcome.logs.length; index++) {
        //obtenemos la primera iteracion del log
        const outcomeLog = outcome.logs[index].toString();

        if(!outcomeLog.includes('EVENT_JSON:')) return

        const parsed = outcomeLog.replace('EVENT_JSON:', '')
        
        //convirtiendo el log en un objeto ValueJSON
        let outcomelogs = json.try_fromString(parsed);
        
        //validamos que se cree un objeto tipo ValueJSON valido a partir del log capturado
        if(!outcomelogs.isOk) return;

        const jsonlog = outcomelogs.value.toObject();
        const eventData = jsonlog.get('data')
        
        if (!eventData) return
        
        const eventArray:JSONValue[] = eventData.toArray()

        const data = eventArray[0].toObject()
        const tokenIds = data.get('token_ids')
        const owner_id = data.get('owner_id')
        
        if (!tokenIds || !owner_id) return
        
        
        const ids:JSONValue[] = tokenIds.toArray()
        
        const tokenId = ids[0].toString()
        
        const serie_id = tokenId.split(":", 1)[0].toString()
        
        // busco la metadata del token en la entidad Serie
        let metadata = Serie.load(serie_id)
        
        //verifico que la metadata exista, de lo contrario no se guarda el nft
        if(!metadata) return
        
        let is_visible = true

        //buscamos si existe un token id
        let nft = Nft.load(tokenId)
        
        // validando que el token id no exista para agregarlo
        if(!nft) {
          //se crea un nevo espacion en memoria de Form asociado al id y se guardan los datos
          nft = new Nft(tokenId)  
          nft.serie_id = serie_id
          nft.owner_id = owner_id.toString()
          nft.fecha = BigInt.fromU64(blockHeader.timestampNanosec)
          nft.is_visible = is_visible
          nft.metadata = serie_id
          nft.save()

          metadata.supply = metadata.supply.plus(BigInt.fromString("1"))
          metadata.save()
        }

        
        let datanft = Datanft.load("1");
        if(!datanft) {
          datanft = new Datanft("1");
          datanft.total_Supply = BigInt.fromI32(0);
          datanft.total_owners = BigInt.fromI32(0);
        }

        datanft.total_Supply = datanft.total_Supply.plus(BigInt.fromI32(1));

        
      }
    }
  }


  if (methodName == 'nft_transfer' || methodName == 'nft_transfer_payout' || methodName == 'nft_transfer_unsafe' || methodName == 'nft_transfer_call' || methodName == 'deliver_gift') {  
    if(outcome.logs.length > 0) {
      //obtenemos la primera iteracion del log
      const outcomeLog = outcome.logs[0].toString();
      const parsed = outcomeLog.replace('EVENT_JSON:', '')  
      //convirtiendo el log en un objeto ValueJSON
      let outcomelogs = json.try_fromString(parsed);
    
      //validamos que se cree un objeto tipo ValueJSON valido a partir del log capturado
      if(!outcomelogs.isOk)  return

      const jsonlog = outcomelogs.value.toObject();
      
      const eventData = jsonlog.get('data')
      if (!eventData) return
      
      const eventArray:JSONValue[] = eventData.toArray()

      const data = eventArray[0].toObject()
      const tokenIds = data.get('token_ids')
      const new_owner_id = data.get('new_owner_id')
      
      if (!tokenIds || !new_owner_id) return
      
      const ids:JSONValue[] = tokenIds.toArray()
      const tokenId = ids[0].toString()

      //buscamos si existe un token id
      let nft = Nft.load(tokenId)
      //validando que el token id exista para actualizar el owner
      if(nft) { 
        nft.owner_id = new_owner_id.toString() 
        nft.save()
      }

    }
  }

  
/*  if (methodName == 'nft_burn') {  
    if(outcome.logs.length > 0) {
      for (let index = 0; index < outcome.logs.length; index++) {
        //obtenemos la primera iteracion del log
        const outcomeLog = outcome.logs[index].toString();

        const parsed = outcomeLog.replace('EVENT_JSON:', '')  
        //convirtiendo el log en un objeto ValueJSON
        let outcomelogs = json.try_fromString(parsed);
      
        //validamos que se cree un objeto tipo ValueJSON valido a partir del log capturado
        if(!outcomelogs.isOk) return

        const jsonlog = outcomelogs.value.toObject();
        
        const eventData = jsonlog.get('data')
        if (!eventData) return
        
        const eventArray:JSONValue[] = eventData.toArray()

        const data = eventArray[0].toObject()
        const tokenIds = data.get('token_ids')
        const owner_id = data.get('owner_id')
        
        if (!tokenIds || !owner_id) return
        
        const ids:JSONValue[] = tokenIds.toArray()
        const tokenId = ids[0].toString()

        //buscamos si existe un token id
        let nft = Nft.load(tokenId)
        //validando que el token id exista para eliminarlo
        if(nft) { 
          nft.delete()
        }
      }
    }
  } */

}