//! Tradução PT-BR da interface, aplicada ao desenhar o texto (ui::BitmapFont).
use std::borrow::Cow;

static EXACT: &[(&str, &str)] = &[
    ("Gnome", "Gnomo"),
    ("Elder Gnome", "Gnomo Ancião"),

    ("This slot has no usable save. Choose Save to create one.", "Este espaço não tem um save utilizável. Escolha Salvar para criar um."),
    ("Return to walking and retry before summoning Cheshire.", "Volte a andar e tente de novo antes de chamar o Cheshire."),
    ("The observatory needs the Lucky Star from the Elder Gnome.", "O observatório precisa da Estrela da Sorte do Gnomo Ancião."),
    ("Step aboard the observatory lift through the open star doors.", "Suba no elevador do observatório pelas portas da estrela abertas."),
    ("The observatory exit needs the Drink Me potion from the Elder Gnome.", "A saída do observatório precisa da poção Beba-me do Gnomo Ancião."),
    ("The globe opens. Drink the potion and follow the White Rabbit into the Pool of Tears.", "O globo se abre. Beba a poção e siga o Coelho Branco até o Lago de Lágrimas."),
    ("The theatre passage is ready. Return through the doors to open the library.", "A passagem do teatro está pronta. Volte pelas portas para abrir a biblioteca."),
    ("The library passage is opening. Find the four flying books.", "A passagem da biblioteca está se abrindo. Encontre os quatro livros voadores."),
    ("The shelves are tipping. Stand clear, then climb the fallen shelves.", "As estantes estão tombando. Afaste-se e depois suba nas estantes caídas."),
    ("Search the other library floors.", "Procure nos outros andares da biblioteca."),
    ("Wake all four flying books before approaching the recipe book.", "Desperte os quatro livros voadores antes de se aproximar do livro de receitas."),
    ("The recipe book has fallen. Return to it on the library's lowest floor.", "O livro de receitas caiu. Volte até ele no andar mais baixo da biblioteca."),
    ("School: follow the corridor left to the theatre.", "Escola: siga o corredor à esquerda até o teatro."),
    ("School: return through the theatre doors to reveal the library passage.", "Escola: volte pelas portas do teatro para revelar a passagem da biblioteca."),
    ("School: reach the fallen recipe book on the lowest library floor.", "Escola: chegue ao livro de receitas caído no andar mais baixo da biblioteca."),
    ("School: cross the flying-book bridge to the large recipe book.", "Escola: atravesse a ponte de livros voadores até o grande livro de receitas."),
    ("School: use the gym lever, climb the bleachers and reach the Elder Gnome.", "Escola: use a alavanca do ginásio, suba a arquibancada e chegue ao Gnomo Ancião."),
    ("School: defeat the three Boojums in the gym. Cards and the thrown Blade can reach them.", "Escola: derrote os três Boojums no ginásio. As Cartas e a Lâmina arremessada os alcançam."),
    ("School: return to the hall, climb the broken central stairs and reach the laboratory and defeat the two guards threatening the Gnome.", "Escola: volte ao saguão, suba a escada central quebrada, chegue ao laboratório e derrote os dois guardas que ameaçam o Gnomo."),
    ("School: collect Jumbogrow from the opened laboratory cabinet.", "Escola: pegue o Jumbogrow no armário aberto do laboratório."),
    ("School: take Jumbogrow through a corridor mirror to the west classrooms and greenhouse lollipop.", "Escola: leve o Jumbogrow por um espelho do corredor até as salas oeste e o pirulito da estufa."),
    ("The lollipop is growing. Wait for it to reach full size.", "O pirulito está crescendo. Espere até ele atingir o tamanho máximo."),
    ("School: collect the grown lollipop.", "Escola: pegue o pirulito crescido."),
    ("School: bring the lollipop back to the Elder Gnome in the laboratory.", "Escola: leve o pirulito de volta ao Gnomo Ancião no laboratório."),
    ("The Gnome is preparing the shrinking potion.", "O Gnomo está preparando a poção de encolher."),
    ("School: collect both the Drink Me potion and Lucky Star near the condenser.", "Escola: pegue a poção Beba-me e a Estrela da Sorte perto do condensador."),
    ("School: the return portal is open. Take the potion and star back to the first school.", "Escola: o portal de volta está aberto. Leve a poção e a estrela de volta à primeira escola."),
    ("Swim through the opened temple passage.", "Nade pela passagem aberta do templo."),
    ("Use the Eye Staff to break the wall leading to the hedge maze. Humpty's secret is optional.", "Use o Cajado do Olho para quebrar o muro que leva ao labirinto de sebes. O segredo do Humpty é opcional."),
    ("Not enough Will / switching to Vorpal Blade", "Vontade insuficiente / trocando para a Lâmina Vorpal"),
    ("Let the current conversation finish first.", "Deixe a conversa atual terminar primeiro."),
    ("The three doors are opening.", "As três portas estão se abrindo."),
    ("Door locked / requires a level event", "Porta trancada / requer um evento da fase"),
    ("Fatal fall / Enter to retry", "Queda fatal / Enter para tentar de novo"),
    ("This level event is not playable yet", "Este evento da fase ainda não é jogável"),
    ("Talk again", "Falar de novo"),
    ("Tap Retry", "Toque em Tentar de novo"),
    ("Tap a toy to equip    Tap outside to return", "Toque num brinquedo para equipar    Toque fora para voltar"),
    ("TOY >", "BRINQUEDO >"),
    ("The path winds through the room. Look around carefully, then choose where to go next.", "O caminho serpenteia pela sala. Olhe ao redor com atenção e escolha para onde ir."),
    ("The Skool window opens on your return. Follow the lower platforms to the portal, then cross the splitting room to Beyond the Wall.", "A janela da Escola se abre na sua volta. Siga as plataformas de baixo até o portal e atravesse a sala que se divide até Além do Muro."),
    ("Strike the Jabberwock in the air. Keep moving when it dives.", "Ataque o Jaguadarte no ar. Continue se movendo quando ele mergulhar."),
    ("Survive the Jabberwock until help arrives. Dodge its fire and defeat the Jabberspawn.", "Sobreviva ao Jaguadarte até a ajuda chegar. Desvie do fogo e derrote as crias do Jaguadarte."),
    ("Survive the ambush until the path to the Sanctum opens.", "Sobreviva à emboscada até o caminho para o Santuário se abrir."),
    ("Use the mirror to find safe stepping stones.", "Use o espelho para achar as pedras seguras."),
    ("The child is holding the plate. Go through the double gate.", "A criança está segurando a placa. Passe pelo portão duplo."),
    ("Swim through the opened tunnel and find the last lever.", "Nade pelo túnel aberto e encontre a última alavanca."),
    ("Turn the mirror. Match each suit to the portrait it replaces, then strike that portrait in its room.", "Gire o espelho. Associe cada naipe ao retrato que ele substitui e golpeie esse retrato na sala dele."),
    ("Red pawn", "Peão vermelho"),
    ("Red knight", "Cavalo vermelho"),
    ("Red bishop", "Bispo vermelho"),
    ("Red rook", "Torre vermelha"),
    ("Touch: Auto", "Toque: Auto"),
    ("Touch: ON", "Toque: LIGADO"),
    ("Touch: OFF (Pad)", "Toque: DESLIGADO (Controle)"),
    ("Touch controls DISABLED (Gamepad mode active).", "Controles de toque DESATIVADOS (modo controle ativo)."),
    ("Select PK3 / ZIP", "Selecionar PK3 / ZIP"),
    ("START GAME", "INICIAR JOGO"),
    ("Starting extraction...", "Iniciando extração..."),
    ("Opening Android system file chooser (.pk3 / .zip)...", "Abrindo o seletor de arquivos do Android (.pk3 / .zip)..."),
    ("Tap 'Choose Game Folder' or 'Select PK3 / ZIP' to import your game, then 'START GAME'.", "Toque em 'Escolher pasta do jogo' ou 'Selecionar PK3 / ZIP' para importar o jogo e depois em 'INICIAR JOGO'."),
    ("Needs: pak0, pak1_large, pak2, pak3, pak4_english, pak5_mod (.pk3)", "Requer: pak0, pak1_large, pak2, pak3, pak4_english, pak5_mod (.pk3)"),
    ("Village Gnome", "Gnomo da Vila"),
    ("Torch Gnome", "Gnomo da Tocha"),
    ("White Rabbit", "Coelho Branco"),
    ("Mad Hatter", "Chapeleiro Louco"),
    ("Mock Turtle", "Tartaruga Falsa"),
    ("Caterpillar", "Lagarta"),
    ("Duchess", "Duquesa"),
    ("Gryphon", "Grifo"),
    ("Queen of Hearts", "Rainha de Copas"),
    ("Talk", "Falar"),
    ("Paused", "Pausado"),
    ("Bleachers extending / climb toward the Elder Gnome", "Arquibancada estendendo / suba até o Gnomo Ancião"),
    // HUD / avisos / objetivos
("Collected {}", "Coletado: {}"),
("Granted {} / Will {}", "Recebido {} / Vontade {}"),
("Sanity {:.0}    Will {:.0}    Primary cost: {:.0}", "Sanidade {:.0}    Vontade {:.0}    Custo primário: {:.0}"),
("Sanity {}", "Sanidade {}"),
("Sanity 100", "Sanidade 100"),
("Harmful liquid / Sanity lost", "Líquido nocivo / Sanidade perdida"),
("Hazard / Sanity lost", "Perigo / Sanidade perdida"),
("Guard struck Alice: {} damage, {} sanity", "Guarda atingiu Alice: {} de dano, {} de sanidade"),
("P or click to resume", "P ou clique para continuar"),
("P to resume film", "P para continuar o filme"),
("Restored at the level entrance", "Restaurado na entrada da fase"),
("Restored to footing before the fall", "Restaurado ao apoio antes da queda"),
("Game saved to {}", "Jogo salvo em {}"),
("Saved {}: {} ({})", "Salvo {}: {} ({})"),
("Opened {}", "Aberto: {}"),
("Not yet available: {}", "Ainda não disponível: {}"),
("Move clear of the wall before switching to walking.", "Afaste-se da parede antes de voltar a andar."),
("Save/load is unavailable in a staged preview", "Salvar/carregar indisponível na prévia"),
("Save loading is unavailable in a staged preview", "Carregar indisponível na prévia"),
("Replace {} with this game?", "Substituir {} por este jogo?"),
("Lift moving / step aboard; it will return if you miss it.", "Elevador em movimento / suba; ele volta se você perder."),
("Listen to the Elder Gnome / E advances dialogue.", "Ouça o Gnomo Ancião / E avança o diálogo."),
("Listen to the Gnome's directions / E advances dialogue.", "Ouça as instruções do Gnomo / E avança o diálogo."),
("Listen to the three notes, then match their order using the levers below.", "Ouça as três notas e repita a ordem usando as alavancas abaixo."),
("Investigate the laboratory.", "Investigue o laboratório."),
("Land on all four chair cushions.", "Pouse nas quatro almofadas das cadeiras."),
("Pull the laboratory lever.", "Puxe a alavanca do laboratório."),
("Pull both levers beside the great gear.", "Puxe as duas alavancas ao lado da grande engrenagem."),
("Reach the lowered clock and stop the machinery.", "Alcance o relógio rebaixado e pare as máquinas."),
("Reach the Staff altar above the pit.", "Alcance o altar do Cajado acima do fosso."),
("Reach the King", "Alcance o Rei"),
("Raise the water", "Eleve a água"),
("Pass the suction mouths and reach the arena.", "Passe pelas bocas de sucção e alcance a arena."),
("Pass through the opened heart gate.", "Atravesse o portão do coração aberto."),
("Leave the second leaf before it sinks. Cross the pads and defeat the house guards.", "Saia da segunda folha antes que afunde. Atravesse as vitórias-régias e derrote os guardas da casa."),
("Ride the final leaf toward the tunnel to Hollow Hideaway.", "Siga na última folha até o túnel para o Esconderijo Oco."),
("Ride the lift into the castle.", "Suba no elevador até o castelo."),
("Ride the airship to the fortress.", "Siga no dirigível até a fortaleza."),
("Ride the minecart. Its transport ends on the far ledge.", "Siga no carrinho de mina. Ele para na borda distante."),
("Ride the lift, then follow the stairs to the observatory globe.", "Suba no elevador e siga a escada até o globo do observatório."),
("Return through the village streets to the Gnome's house.", "Volte pelas ruas da vila até a casa do Gnomo."),
("Return: climb the library to the star doors. Your Lucky Star opens the lift.", "Retorno: escale a biblioteca até as portas da estrela. Sua Estrela da Sorte abre o elevador."),
("Lucky Star", "Estrela da Sorte"),
("Eye Staff", "Cajado do Olho"),
("Pocket Watch", "Relógio de Bolso"),
("Jack Bomb", "Bomba Jack"),
("MAD HATTER", "CHAPELEIRO LOUCO"),
("QUEEN OF HEARTS", "RAINHA DE COPAS"),
("RED KING", "REI VERMELHO"),
("DUCHESS / PHASING", "DUQUESA / TRANSIÇÃO"),
("JABBERWOCK / SURVIVE {:.0}s", "JABBERWOCK / SOBREVIVA {:.0}s"),
("March Hare", "Lebre de Março"),
("HUD preview / sample resources and all toys", "Prévia do HUD / recursos de exemplo e todos os brinquedos"),
("Larva encounter / WASD move, mouse attack / Home retry", "Encontro com larvas / WASD move, mouse ataca / Home repete"),
("Left stick: move   {}: jump / rise   {}: dive", "Analógico esq.: mover   {}: pular / subir   {}: mergulhar"),
("Left stick: move   Jump/Rise: jump or swim up   Dive: crouch or dive", "Analógico esq.: mover   Pular/Subir: pular ou nadar para cima   Mergulhar: agachar ou mergulhar"),
("Mouse / look keys", "Mouse / teclas de olhar"),
("Right side drag", "Arrastar no lado direito"),
("Right stick", "Analógico direito"),
    ("Game saved", "Jogo salvo"),
    ("Game saved / F9 to load", "Jogo salvo / F9 para carregar"),
    ("Progress saved", "Progresso salvo"),
    ("Checkpoint saved", "Checkpoint salvo"),
    ("Checkpoint restored", "Checkpoint restaurado"),
    ("Settings saved", "Configurações salvas"),
    ("Automatic save", "Salvamento automático"),
    ("Quick save", "Salvamento rápido"),
    ("Load Game", "Carregar jogo"),
    ("Save Game", "Salvar jogo"),
    ("A or Start to resume", "A ou Start para continuar"),
    ("A little guidance", "Uma pequena ajuda"),
    ("Sound muted - M to restore", "Som mudo - M para restaurar"),
    ("Teleported", "Teletransportada"),
    ("Teleport destination is obstructed", "Destino do teletransporte bloqueado"),
    ("Card guard struck you", "Um guarda-carta acertou você"),
    ("Harmful liquid", "Líquido nocivo"),
    ("Hazard", "Perigo"),
    ("Hard landing", "Queda forte"),
    ("Sanity lost", "Sanidade perdida"),
    ("Enemy defeated", "Inimigo derrotado"),
    ("Ant defeated", "Formiga derrotada"),
    ("Card guard defeated", "Guarda-carta derrotado"),
    ("Fire Imp defeated", "Diabrete de fogo derrotado"),
    ("Clockwork Automaton defeated", "Autômato de corda derrotado"),
    ("Gym lever activated", "Alavanca do ginásio ativada"),
    ("Demon Dice are still active or recovering", "Os Dados Demoníacos ainda estão ativos ou recarregando"),
    ("This toy's attacks are not yet available", "Os ataques deste brinquedo ainda não estão disponíveis"),
    ("A power-up is already active; wait for it to expire.", "Já há um poder ativo; espere ele acabar."),
    ("Cheshire is unavailable during this scene.", "O Gato de Cheshire não está disponível nesta cena."),
    ("Cheshire will return in a moment.", "O Gato de Cheshire volta em instantes."),
    ("Campaign complete / Continue replays the final encounter", "Campanha concluída / Continuar repete o confronto final"),
    ("Campaign complete. Load a checkpoint or start a new game.", "Campanha concluída. Carregue um checkpoint ou comece um novo jogo."),
    ("Choose a manual slot or the quick save.", "Escolha um slot manual ou o salvamento rápido."),
    ("That control is reserved. Choose another.", "Esse controle é reservado. Escolha outro."),
    ("Entrance is obstructed / choose another level with Tab", "Entrada bloqueada / escolha outro nível com Tab"),
    ("Lost beyond the level / Enter to retry", "Perdida fora do nível / Enter para tentar de novo"),
    ("Swimming / Space rise / Ctrl dive / look and W to swim", "Nadando / Espaço sobe / Ctrl mergulha / olhe e W para nadar"),
    ("Minecart / hold on", "Vagonete / segure firme"),
    ("Airship / departing for the fortress", "Dirigível / partindo para a fortaleza"),
    ("Key secured / return to the Gnome's house", "Chave obtida / volte à casa do Gnomo"),
    ("The Gnome is waiting / enter his house", "O Gnomo está esperando / entre na casa dele"),
    ("Collected the Gnome's key / the upper door opens", "Chave do Gnomo coletada / a porta de cima se abre"),
    ("Lucky Star used / step aboard the observatory lift", "Estrela da Sorte usada / suba no elevador do observatório"),
    ("Recipe found / continuing to the next part of the school", "Receita encontrada / seguindo para a próxima parte da escola"),
    ("Already used / reset the sequence at the upper lever", "Já usada / reinicie a sequência na alavanca de cima"),
    ("D-pad: select    A: equip    B: return", "D-pad: selecionar    A: equipar    B: voltar"),
    ("Console   ~ / Esc to close", "Console   ~ / Esc para fechar"),
    ("first person", "primeira pessoa"),
    ("Drag any button or joystick to move. Tap to select & adjust size/visibility.", "Arraste qualquer botão ou joystick para mover. Toque para selecionar e ajustar tamanho/visibilidade."),
    ("All touch buttons set to Visible.", "Todos os botões de toque visíveis."),
    ("All touch controls reset to defaults.", "Controles de toque restaurados ao padrão."),
    ("Always On", "Sempre ligado"),
    // Ações (bindings / toque)
    ("Alternate attack", "Ataque alternativo"),
    ("Change view", "Mudar visão"),
    ("Choose level", "Escolher nível"),
    ("Cheshire hint", "Dica do Cheshire"),
    ("Alternate Attack (ATK 2)", "Ataque alternativo (ATQ 2)"),
    ("Camera View Toggle (VIEW)", "Alternar câmera (VISTA)"),
    ("Cheshire Cat Hint (CAT)", "Dica do Gato (GATO)"),
    ("Help Overlay (HELP)", "Ajuda (AJUDA)"),
    // Prompts de tecla
    ("E  extend gym bleachers", "E  estender arquibancada"),
    ("E  open door", "E  abrir porta"),
    ("E  play lever note", "E  tocar nota da alavanca"),
    ("E  Pull lever", "E  puxar alavanca"),
    ("E  reset and replay the three notes", "E  reiniciar e repetir as três notas"),
    ("E / Turn the mirror", "E / girar o espelho"),
    ("E grab rope", "E agarrar corda"),
    ("E release rope / Space up / Ctrl down / WASD swing", "E soltar corda / Espaço sobe / Ctrl desce / WASD balança"),
    ("E: open door", "E: abrir porta"),
    ("E: use lever", "E: usar alavanca"),
    // Itens / personagens
    ("Drink Me potion", "Poção Beba-me"),
    ("Cheshire Cat", "Gato de Cheshire"),
    ("Bill the Lizard", "Bill, o Lagarto"),
    ("Chess King", "Rei do Xadrez"),
    // Mundo / objetivos
    ("A hidden panel opens in the library.", "Um painel secreto se abre na biblioteca."),
    ("A note echoes through the hall.", "Uma nota ecoa pelo salão."),
    ("A new chapter is ready. Your toys and progress have been carried into this visit. Explore the room at your own pace; you can open the main menu whenever you need to save, load, or adjust your controls.", "Um novo capítulo está pronto. Seus brinquedos e progresso foram mantidos. Explore no seu ritmo; abra o menu principal quando precisar salvar, carregar ou ajustar os controles."),
    ("Approach the house and speak to Bill.", "Aproxime-se da casa e fale com o Bill."),
    ("Avoid the Clockworks and the falling tea. The Hatter will return with the clock.", "Evite os Autômatos e o chá que cai. O Chapeleiro voltará com o relógio."),
    ("Board the fallen leaves to follow the river. Use the banks between rides; more leaves follow if you miss one.", "Suba nas folhas caídas para seguir o rio. Use as margens entre elas; se perder uma, vêm outras."),
    ("Clear the galleries and ring the bell", "Limpe as galerias e toque o sino"),
    ("Climb the fallen spikes to the mushroom.", "Escale os espinhos caídos até o cogumelo."),
    ("Climb the riverbank and speak to the Mock Turtle at the top.", "Suba a margem do rio e fale com a Tartaruga Falsa lá em cima."),
    ("Climb to the high walkway above the globe to use the Drink Me potion.", "Suba à passarela acima do globo para usar a poção Beba-me."),
    ("Collect the blade from the centre of the clock.", "Pegue a lâmina no centro do relógio."),
    ("Collect the fallen eye to obtain the Eye Staff.", "Pegue o olho caído para obter o Cetro do Olho."),
    ("Complete the bishop lesson", "Conclua a lição do bispo"),
    ("Complete the knight lesson", "Conclua a lição do cavalo"),
    ("Continue through the castle gate.", "Siga pelo portão do castelo."),
    ("Continue through the caverns to the Jabberwock's lair.", "Siga pelas cavernas até o covil do Jabberwock."),
    ("Continue through the doors beyond the Watch.", "Siga pelas portas além do Relógio."),
    ("Continue through the opened wall into the hedge maze.", "Passe pela parede aberta até o labirinto de sebes."),
    ("Cross the battlefield and reach the castle approach.", "Atravesse o campo de batalha até a entrada do castelo."),
    ("Cross the bridge and approach the blade at the centre of the clock.", "Atravesse a ponte e chegue à lâmina no centro do relógio."),
    ("Cross the extending bridge.", "Atravesse a ponte que se estende."),
    ("Cross the lava caverns and find the Caterpillar.", "Atravesse as cavernas de lava e encontre a Lagarta."),
    ("Cross the vents and find the Caterpillar.", "Atravesse as fendas de vapor e encontre a Lagarta."),
    ("Cross their bridge to the large recipe book.", "Atravesse a ponte deles até o grande livro de receitas."),
    ("Defeat both Tweedles.", "Derrote os dois Tweedles."),
    ("Defeat the Hatter. Dodge his cups and syringes, and keep clear of his cane.", "Derrote o Chapeleiro. Desvie das xícaras e seringas e fique longe da bengala."),
    ("Defeat the Queen. Keep moving to escape her magic; essence appears at the back of the hall.", "Derrote a Rainha. Continue se movendo para fugir da magia; a essência aparece no fundo do salão."),
    ("Defeat the Queen's true form. Destroy tentacles to disable attacks; circle the platforms for essence.", "Derrote a forma verdadeira da Rainha. Destrua os tentáculos para anular ataques; circule as plataformas para pegar essência."),
    ("Defeat the Red King. Dodge his magic and collect the returning essence.", "Derrote o Rei Vermelho. Desvie da magia e pegue a essência que volta."),
    ("Descend to the Elder Gnome, then follow the path to the minecart. E grabs/releases ropes; Space/Ctrl climbs; WASD swings.", "Desça até o Gnomo Ancião e siga até o vagonete. E agarra/solta cordas; Espaço/Ctrl sobe e desce; WASD balança."),
    ("Dodge the Centipede. Strike the glowing weak spot as it rears up to crush you.", "Desvie da Centopeia. Acerte o ponto fraco brilhante quando ela se erguer para esmagar você."),
    ("Enter the castle", "Entre no castelo"),
    ("Enter the Caterpillar's portal.", "Entre no portal da Lagarta."),
    ("Enter the Hatter's arena.", "Entre na arena do Chapeleiro."),
    ("Enter the moving portal", "Entre no portal em movimento"),
    ("Enter the throne room.", "Entre na sala do trono."),
    ("Enter the tower.", "Entre na torre."),
    ("Escape with the Gryphon.", "Fuja com o Grifo."),
    ("Explore the library. The flying books can make a path to the recipe.", "Explore a biblioteca. Os livros voadores podem formar um caminho até a receita."),
    ("Face the Red King.", "Enfrente o Rei Vermelho."),
    ("Find the child in the maze and guide him back to the pressure plate.", "Encontre a criança no labirinto e leve-a de volta à placa de pressão."),
    ("Find the child in the maze. The nearby portal is a shortcut.", "Encontre a criança no labirinto. O portal próximo é um atalho."),
    ("Find the imprisoned Gryphon.", "Encontre o Grifo aprisionado."),
    ("Find the lever below the maze.", "Encontre a alavanca sob o labirinto."),
    ("Find the third Demon Die and continue through the red castle.", "Encontre o terceiro Dado Demoníaco e siga pelo castelo vermelho."),
    ("Finish the wounded Jabberwock. Dodge its breath and stay clear of its claws.", "Acabe com o Jabberwock ferido. Desvie do sopro e fique longe das garras."),
    ("Follow the bank and lily pads to the first ride leaf. The fish are waiting below.", "Siga pela margem e vitórias-régias até a primeira folha. Os peixes esperam lá embaixo."),
    ("Follow the chess path through the raised gate.", "Siga o caminho do xadrez pelo portão erguido."),
    ("Follow the east bank to the second leaf. Stay out of the pond.", "Siga pela margem leste até a segunda folha. Fique fora do lago."),
    ("Follow the forest path toward the Centipede.", "Siga a trilha da floresta até a Centopeia."),
    ("Follow the ice path. Keep clear of falling rocks.", "Siga o caminho de gelo. Cuidado com as pedras que caem."),
    ("Follow the maze to the child and the pressure plate.", "Siga o labirinto até a criança e a placa de pressão."),
    ("Follow the opened cavern to the Caterpillar's clearing.", "Siga pela caverna aberta até a clareira da Lagarta."),
    ("Follow the Queen towards the portal.", "Siga a Rainha até o portal."),
    ("Follow the slide into the arena.", "Desça o escorregador até a arena."),
    ("Follow the third leaf past the posts. Jump and grab the rope before the waterfall, then climb to the bank.", "Siga a terceira folha passando pelos postes. Pule e agarre a corda antes da cachoeira e suba até a margem."),
    ("Follow the Turtle; touch his bubble trail to refill your air.", "Siga a Tartaruga; toque no rastro de bolhas para recuperar o ar."),
    ("Follow the upper bank and board the final fallen leaf.", "Siga pela margem de cima e suba na última folha."),
    ("Follow the upper castle route to the King's drawbridge.", "Siga a rota alta do castelo até a ponte levadiça do Rei."),
    ("Follow the upper wall to the Skool. Run and jump from the raised edge through its open window.", "Siga pelo muro alto até a Escola. Corra e pule da borda elevada pela janela aberta."),
    ("Ride a moving clock platform to collect the Pocket Watch.", "Suba numa plataforma do relógio para pegar o Relógio de Bolso."),
    ("Stand back while the Hatter breaks down.", "Afaste-se enquanto o Chapeleiro desmorona."),
    ("Listen to the Cheshire Cat.", "Ouça o Gato de Cheshire."),
    ("Watch the royal execution.", "Assista à execução real."),
    ("The battle is over.", "A batalha acabou."),
    ("The ceiling is collapsing.", "O teto está desabando."),
    ("The Centipede has set a trap.", "A Centopeia armou uma armadilha."),
    ("The drawbridge is lowering.", "A ponte levadiça está descendo."),
    ("The hatch swings open.", "A escotilha se abre."),
    ("The Heart of Darkness awakens.", "O Coração das Trevas desperta."),
    ("The Jabberwock approaches.", "O Jabberwock se aproxima."),
    ("The nightmare is ending.", "O pesadelo está acabando."),
    ("The Queen reveals herself.", "A Rainha se revela."),
    ("Lever sequence reset. Listen and try again.", "Sequência das alavancas reiniciada. Ouça e tente de novo."),
    ("Watch the correct door and try again.", "Observe a porta certa e tente de novo."),
    ("Watch the open door, then follow it through the shuffle.", "Observe a porta aberta e acompanhe-a no embaralhamento."),
    ("The room is rising. Climb the moving steps to the upper door.", "A sala está subindo. Escale os degraus móveis até a porta de cima."),
    ("Scene skipped", "Cena pulada"),
    ("A scene is playing / hold Enter to skip", "Cena em andamento / segure Enter para pular"),
    // Armas (WEAPONS.1)
    ("Vorpal Blade", "Lâmina Vorpal"),
    ("Cards", "Cartas"),
    ("Croquet Mallet", "Taco de Croquet"),
    ("Jackbomb", "Caixa-Bomba"),
    ("Ice Wand", "Varinha de Gelo"),
    ("Jacks", "Jacks"),
    ("Demon Dice", "Dados Demoníacos"),
    ("Jabberwock's Eye Staff", "Cajado do Olho de Jaguadarte"),
    ("Blunderbuss", "Bacamarte"),
    ("Deadtime Watch", "Relógio do Tempo Morto"),
    // Itens e poderes
    ("Sanity shard", "Fragmento de Sanidade"),
    ("Vial of Will", "Frasco de Vontade"),
    ("Meta-essence", "Metaessência"),
    ("Rage Box", "Caixa da Fúria"),
    ("Grasshopper Tea", "Chá de Gafanhoto"),
    ("Darkened Looking Glass", "Espelho Escurecido"),
    ("Sanity full", "Sanidade cheia"),
    ("Will full", "Vontade cheia"),
    ("Sanity and Will full", "Sanidade e Vontade cheias"),
    ("Pocket Watch unavailable", "Relógio de Bolso indisponível"),
    ("Pocket Watch is recharging", "Relógio de Bolso recarregando"),
    ("Not enough Will", "Vontade insuficiente"),
    ("Unarmed", "Desarmada"),
    // HUD
    ("Sanity lost", "Sanidade perdida"),
    ("Tap Retry - retry", "Toque em Repetir - tentar de novo"),
    ("Menu - load a game or change options", "Menu - carregar jogo ou mudar opções"),
    ("Start - load a game or change options", "Start - carregar jogo ou mudar opções"),
    ("Esc - load a game or change options", "Esc - carregar jogo ou mudar opções"),
    ("Alice's Toys", "Brinquedos de Alice"),
    ("Tap a toy to equip", "Toque num brinquedo para equipar"),
    ("Tap outside to return", "Toque fora para voltar"),
    ("D-pad: select", "D-pad: selecionar"),
    ("A: equip", "A: equipar"),
    ("B: return", "B: voltar"),
    ("Arrows / click: select", "Setas / clique: selecionar"),
    ("Selected", "Selecionado"),
    ("Unavailable", "Indisponível"),
    ("Pocket Watch - stops time", "Relógio de Bolso - para o tempo"),
    ("Find a toy to equip", "Encontre um brinquedo para equipar"),
    ("Encounter", "Confronto"),
    // --- tradução anterior ---
    // Menu principal / confirmações
    ("Chapters", "Capítulos"), ("Yes", "Sim"), ("No", "Não"),
    ("Previous", "Anterior"), ("Next", "Próximo"), ("Reset", "Restaurar"),
    ("Apply", "Aplicar"), ("Cancel", "Cancelar"),
    ("Replace this session with the saved game?", "Substituir esta sessão pelo jogo salvo?"),
    ("Your current game will be saved automatically.", "Seu jogo atual será salvo automaticamente."),
    ("Choose No to return.", "Escolha Não para voltar."),
    // Vídeo / áudio / jogo
    ("Window Resolution", "Resolução da janela"), ("Display Mode", "Modo de exibição"),
    ("Performance Profile", "Perfil de desempenho"),
    ("Balances detail, lights & shadows.", "Equilibra detalhes, luzes e sombras."),
    ("Fullscreen", "Tela cheia"), ("Windowed", "Janela"),
    ("Music Volume", "Volume da música"), ("Effects Volume", "Volume dos efeitos"), ("Mute", "Mudo"),
    ("Sensitivity", "Sensibilidade"), ("Invert Mouse", "Inverter mouse"),
    ("Camera Distance", "Distância da câmera"), ("Always Run", "Sempre correr"),
    ("Subtitles", "Legendas"),
    ("1 Video   2 Audio   3 Controls   4 Game", "1 Vídeo   2 Áudio   3 Controles   4 Jogo"),
    // Controles / menus
    ("Backward", "Para trás"),
("Dive / descend", "Mergulhar / descer"),
("Jump / climb", "Pular / escalar"),
("Run / walk", "Correr / andar"),
("Look up", "Olhar para cima"),
("Look down", "Olhar para baixo"),
("Look left", "Olhar à esquerda"),
("Look right", "Olhar à direita"),
("Mute sound", "Silenciar som"),
("Next toy", "Próximo brinquedo"),
("Previous toy", "Brinquedo anterior"),
("Primary attack", "Ataque primário"),
("Quick load", "Carregamento rápido"),
("Restart level", "Reiniciar fase"),
("Retry / skip scene", "Repetir / pular cena"),
("Safe footing", "Apoio firme"),
("Face buttons", "Botões de ação"),
("Next >", "Próximo >"),
("Down >", "Abaixo >"),
(".. Parent", ".. Pasta acima"),
("{} [OFF]", "{} [DESL]"),
("No action selected", "Nenhuma ação selecionada"),
("not saved yet", "ainda não salvo"),
("Default layout restored (not saved yet).", "Layout padrão restaurado (ainda não salvo)."),
("Press a controller button now (Start to stop)…", "Aperte um botão do controle (Start para parar)…"),
("Save & Close", "Salvar e fechar"),
("Reset {} to default.", "Restaurar {} ao padrão."),
("Opac. {:.0}%", "Opac. {:.0}%"),
("Interact / Use (USE)", "Interagir / Usar (USAR)"),
("Jump / Climb / Rise (JUMP)", "Pular / Escalar / Subir (PULAR)"),
("Movement Joystick (STICK)", "Joystick de movimento (STICK)"),
("Next Toy (TOY >)", "Próximo brinquedo (BRINQ >)"),
("Previous Toy (< TOY)", "Brinquedo anterior (< BRINQ)"),
("Pause / Main Menu (MENU)", "Pausa / Menu principal (MENU)"),
("Quick Load (LOAD)", "Carregamento rápido (CARREGAR)"),
("Quick Save (SAVE)", "Salvamento rápido (SALVAR)"),
("Run / Sprint Lock (RUN)", "Correr / Travar corrida (CORRER)"),
("Safe Footing (FOOT)", "Apoio firme (APOIO)"),
("FPS Limit:", "Limite de FPS:"),
("FPS Limit: {}", "Limite de FPS: {}"),
("Preset: {}", "Predefinição: {}"),
("Overlay: {}", "Sobreposição: {}"),
("Current Folder: {}", "Pasta atual: {}"),
("Download game", "Baixar jogo"),
("Edit Touch HUD", "Editar HUD de toque"),
("Game data ready in {}", "Dados do jogo prontos em {}"),
("Game data ready in {}. Select FPS / Preset and tap START GAME.", "Dados do jogo prontos em {}. Escolha FPS / Predefinição e toque em INICIAR JOGO."),
("Game files folder: {}", "Pasta dos arquivos do jogo: {}"),
("Missing game files: {}. Use Choose Game Folder or Select PK3 / ZIP.", "Arquivos do jogo ausentes: {}. Use Escolher pasta do jogo ou Selecionar PK3 / ZIP."),
("Opening Android system folder chooser...", "Abrindo o seletor de pastas do Android..."),
("Opening archive.org ...", "Abrindo archive.org ..."),
(" (backup)", " (cópia)"),
("Map starts a fresh visit. While alive, restart recovers the entrance. After death, retry restores the last save or visit start.", "O mapa inicia uma nova visita. Vivo, reiniciar volta à entrada. Após morrer, tentar de novo restaura o último save ou o início da visita."),
    ("Touch Controls", "Controles de toque"), ("Controller", "Controle"),
    ("Keyboard / Mouse", "Teclado / Mouse"), ("Touch Sensitivity", "Sensibilidade do toque"),
    ("Invert Touch Look", "Inverter visão (toque)"), ("Button Size", "Tamanho dos botões"),
    ("HUD Opacity", "Opacidade do HUD"), ("Left-Handed Layout", "Layout para canhotos"),
    ("Look Sensitivity", "Sensibilidade da visão"), ("Invert Look", "Inverter visão"),
    ("Stick Deadzone", "Zona morta do analógico"),
    ("Press a button   Start / Esc cancel   Delete clear", "Aperte um botão   Start / Esc cancela   Delete limpa"),
    ("Press a key, mouse button or wheel   Esc cancels", "Aperte uma tecla, botão ou roda do mouse   Esc cancela"),
    ("D-pad / stick: browse   A: select   B: back", "D-pad / analógico: navegar   A: selecionar   B: voltar"),
    ("Arrows adjust   Enter select   Esc cancel", "Setas ajustam   Enter seleciona   Esc cancela"),
    ("Tab: chapters", "Tab: capítulos"),
    ("Tab: chapters   Start / B: back to game", "Tab: capítulos   Start / B: voltar ao jogo"),
    ("Tab: chapters   Esc: back to game", "Tab: capítulos   Esc: voltar ao jogo"),
    // Saves
    ("No preview", "Sem prévia"), ("Unknown", "Desconhecido"),
    ("Unreadable save", "Save ilegível"), ("Empty slot", "Espaço vazio"),
    // Capítulos
    ("Chapters of Wonderland", "Capítulos do País das Maravilhas"),
    ("D-pad / left stick: browse   A: begin   B: return", "D-pad / analógico: navegar   A: começar   B: voltar"),
    ("Click: select   Double-click: begin   Wheel / arrows: browse", "Clique: selecionar   Duplo clique: começar   Roda / setas: navegar"),
    ("Begin / A", "Começar / A"), ("Begin / Enter", "Começar / Enter"),
    ("Return / B", "Voltar / B"), ("Return / Tab / Esc", "Voltar / Tab / Esc"),
    // Jogo
    ("Paused", "Pausado"),
    ("Opening Wonderland...", "Abrindo o País das Maravilhas..."),
    ("Game restored / P or click to resume", "Jogo restaurado / P ou clique para continuar"),
    ("Previous save restored / P or click to resume", "Save anterior restaurado / P ou clique para continuar"),
    ("A saved game could not be read. F9 retries quick load.", "Não foi possível ler um jogo salvo. F9 tenta o carregamento rápido de novo."),
    ("Controller disconnected / reconnect or use keyboard and mouse", "Controle desconectado / reconecte ou use teclado e mouse"),
    ("Opening film unavailable / continuing to Wonderland", "Vídeo de abertura indisponível / seguindo para o País das Maravilhas"),
    ("Saving is unavailable in a staged preview", "Salvar não está disponível nesta prévia"),
    ("Retry or load a game before saving", "Tente de novo ou carregue um jogo antes de salvar"),
    ("Loading is unavailable in a staged preview", "Carregar não está disponível nesta prévia"),
    ("New Game is unavailable in a staged preview", "Novo jogo não está disponível nesta prévia"),
    // Objetivos (story.rs)
    ("Explore the east gymnasium. E uses its lever; the Elder Gnome waits above.", "Explore o ginásio leste. E aciona a alavanca; o Gnomo Ancião espera lá em cima."),
    ("Gather the ingredients and help the Elder Gnome prepare the potion.", "Reúna os ingredientes e ajude o Gnomo Ancião a preparar a poção."),
    ("Explore the village; approach its gnomes and press E to talk.", "Explore a vila; chegue perto dos gnomos e aperte E para conversar."),
    // Fases (campaign.rs) — ajuste para os nomes da tradução oficial, se houver
    ("Dementia - opening village", "Demência - vila inicial"),
    ("Fortress of Doors", "Fortaleza das Portas"), ("Beyond the Wall", "Além do Muro"),
    ("Skool Daze", "Dias de Escola"), ("Skool's Out", "Fim das Aulas"),
    ("Pool of Tears", "Lago de Lágrimas"), ("Hollow Hideaway", "Esconderijo Oco"),
    ("Just Desserts - Duchess encounter", "Sobremesa Merecida - encontro com a Duquesa"),
    ("Wholly Morel Ground", "Solo Sagrado dos Cogumelos"), ("Dry Landing", "Pouso Seco"),
    ("Herbaceous Border", "Canteiro de Ervas"), ("Rolling Stones", "Pedras Rolantes"),
    ("Icy Reception", "Recepção Gelada"), ("Fungiferous Flora", "Flora Fungífera"),
    ("Centipede's Sanctum", "Santuário da Centopeia"), ("Caterpillar's Plot", "Terreno da Lagarta"),
    ("Pale Realm", "Reino Pálido"), ("Castling", "Roque"), ("Checkmate in Red", "Xeque-mate em Vermelho"),
    ("Mirror Image", "Imagem no Espelho"), ("Crazed Clockwork", "Engrenagem Enlouquecida"),
    ("About Face", "Meia-Volta"), ("Burning Curiosity", "Curiosidade Ardente"),
    ("Jabberwock's Lair", "Covil do Jabberwock"), ("Majestic Maze", "Labirinto Majestoso"),
    ("Airborne Terror", "Terror Aéreo"), ("Mystifying Madness", "Loucura Misteriosa"),
    ("Water Logged", "Encharcada"), ("Labyrinthine Revenge", "Vingança Labiríntica"),
    ("Machinations", "Maquinações"), ("Royal Rage", "Fúria Real"),
    ("Battle Royale", "Batalha Real"), ("Ascension", "Ascensão"),
    ("Castle Keep", "Torre do Castelo"), ("Heart of Darkness - finale", "Coração das Trevas - final"),

    // Que faltava mas aparece no jogo
    ("Activated {}", "Ativado: {}"),
("Air  {:.0}", "Ar  {:.0}"),
("{} dice", "{} dados"),
("{} - retry", "{} - tentar de novo"),
("{} defeated", "{} derrotado"),
("{} or click to resume", "{} ou clique para continuar"),
("{}: look   {}: change view", "{}: olhar   {}: mudar visão"),
("{}: menus   {}: chapters   {}: help", "{}: menus   {}: capítulos   {}: ajuda"),
("{}: recover    Save / Load in the main menu", "{}: recuperar    Salvar / Carregar no menu principal"),
("{}: Cheshire hints    {}: advance dialogue", "{}: dicas do Cheshire    {}: avançar diálogo"),
("{} / {}: primary / alternate attack", "{} / {}: ataque primário / alternativo"),
("{} / {}: primary / alternate attack (drag to aim)", "{} / {}: ataque primário / alternativo (arraste para mirar)"),
("{} / {}: toy   {}: inventory", "{} / {}: brinquedo   {}: inventário"),
("{} - {} visits   ({} to change)", "{} - {} visitas   ({} para mudar)"),
("Arrows / click: select    {} / Esc: return", "Setas / clique: selecionar    {} / Esc: voltar"),
("Break the cell clocks: {}/8.", "Quebre os relógios das celas: {}/8."),
("Collect the Cards, pass the guards and find the Gnome's key.", "Pegue as Cartas, passe pelos guardas e encontre a chave do Gnomo."),
("Climb the rising air, pass the gates and reach the castle lift.", "Suba nas correntes de ar, passe pelos portões e alcance o elevador do castelo."),
("Continue to the Queen's Lair.", "Siga para o Covil da Rainha."),
("the Gnome's key", "a chave do Gnomo"),
("All toys and Turtle Shell granted / Sanity 100 / Will 100", "Todos os brinquedos e o Casco de Tartaruga liberados / Sanidade 100 / Vontade 100"),
("All toys granted / resources unchanged", "Todos os brinquedos liberados / recursos inalterados"),
("Attack cancelled / resource unavailable", "Ataque cancelado / recurso indisponível"),
("Combat preview / 1 Blade, 2 Cards, 3 Mallet / Home retry", "Prévia de combate / 1 Lâmina, 2 Cartas, 3 Marreta / Home repete"),
("Campaign: {} completed visits / {} cached visits", "Campanha: {} visitas concluídas / {} em cache"),
("Choose Easy, Normal, Hard or Nightmare", "Escolha Fácil, Normal, Difícil ou Pesadelo"),
("Autosave", "Salvamento automático"),
("Apr", "abr"),
("Aug", "ago"),
("Actions", "Ações"),
("Attack", "Ataque"),
("Back", "Voltar"),
("Backward", "Para trás"),
("Balanced", "Equilibrado"),
("Could not save", "Não foi possível salvar"),
("Controller mapping saved.", "Mapeamento do controle salvo."),
("Controller Setup", "Configurar controle"),
("Controller changes discarded.", "Alterações do controle descartadas."),
("Controls:", "Controles:"),
("Check Updates", "Verificar atualizações"),
("Checking for updates on GitHub...", "Verificando atualizações no GitHub..."),
("Choose Game Folder", "Escolher pasta do jogo"),
("Copy From This Folder", "Copiar desta pasta"),
("Credits", "Créditos"),
("Crouch / Dive (CROUCH)", "Agachar / Mergulhar (AGACHAR)"),
("Chapters / Map (MAP)", "Capítulos / Mapa (MAPA)"),
("< TOY", "< BRINQ"),
("< Prev", "< Anterior"),
("< Up", "< Acima"),
("ATK 1", "ATQ 1"),
("ATK 2", "ATQ 2"),
    ("E  close door", "E  fechar porta"),
    ("This door opens when the current quest is complete", "Esta porta se abre quando a missão atual for concluída"),
    ("The key opened the upper door. Climb the hall and enter the return portal.", "A chave abriu a porta de cima. Suba o salão e entre no portal de retorno."),
    ("Game restored", "Jogo restaurado"),
    ("Previous save restored", "Save anterior restaurado"),
    ("Collected the Gnome's key", "Chave do Gnomo coletada"),
    ("the upper door opens", "a porta de cima se abre"),
];

static PREFIX: &[(&str, &str)] = &[
    ("Time stopped  ", "Tempo parado  "),
    ("Watch ready in ", "Relógio pronto em "),
    ("Air  ", "Ar  "),
    ("Collected ", "Coletado: "),
    ("Load failed: ", "Falha ao carregar: "),
    ("Sanity ", "Sanidade "),
    ("Will ", "Vontade "),
    ("Primary cost: ", "Custo primário: "),
    // --- tradução anterior ---
    ("Overlay: ", "Sobreposição: "), ("Preset: ", "Predefinição: "),
    ("FPS Limit: ", "Limite de FPS: "), ("Saving to ", "Salvando em "),
    ("Could not restore save: ", "Não foi possível restaurar o save: "),
    ("Could not preserve the current game: ", "Não foi possível guardar o jogo atual: "),
];

static SUFFIX: &[(&str, &str)] = &[
    (" / Esc: return", " / Esc: voltar"),
    (" - retry", " - tentar de novo"),
    (": primary / alternate attack", ": ataque primário / alternativo"),
    (" dice", " dados"),
];

static PART: &[(&str, &str)] = &[
    (" visits   (", " visitas   ("), (" to change)", " para mudar)"),
    ("D / click", "D / clique"), ("  next line", "  próxima fala"),
];

fn tr_base(text: &str) -> Cow<'_, str> {
    // AUTO-EXATO-INICIO: a frase inteira tem prioridade
    if let Some(&(_, pt)) = EXACT.iter().find(|(en, _)| *en == text) {
        return Cow::Borrowed(pt);
    }
    // AUTO-EXATO-FIM
    if text.contains(" / ") {
        let parts: Vec<String> = text.split(" / ").map(|p| tr(p).into_owned()).collect();
        return Cow::Owned(parts.join(" / "));
    }
    let trimmed = text.trim();
    if trimmed.len() != text.len() && !trimmed.is_empty() {
        return Cow::Owned(text.replace(trimmed, &tr(trimmed)));
    }
    // Texto não ASCII já está traduzido (ou vem dos arquivos do jogo).
    if text.is_empty() || !text.is_ascii() {
        return Cow::Borrowed(text);
    }
    // Dicas com várias partes separadas por 4 espaços.
    if text.contains("    ") {
        return Cow::Owned(text.split("    ").map(|p| tr(p).into_owned()).collect::<Vec<_>>().join("    "));
    }
    if let Some(base) = text.strip_suffix(" - return") {
        return Cow::Owned(format!("{} - retorno", tr(base)));
    }
    if let Some(d) = text.strip_prefix("Start a new ").and_then(|s| s.strip_suffix(" game?")) {
        return Cow::Owned(format!("Iniciar novo jogo ({})?", tr(d)));
    }
    if let Some(s) = text.strip_prefix("Replace ").and_then(|s| s.strip_suffix(" with this game?")) {
        return Cow::Owned(format!("Substituir {} por este jogo?", tr(s)));
    }
    if let Some(k) = text.strip_prefix("Hold ").and_then(|s| s.strip_suffix(" to skip scene")) {
        return Cow::Owned(format!("Segure {k} para pular a cena"));
    }
    if let Some(r) = text.strip_prefix("Break the cell clocks: ") {
        return Cow::Owned(format!("Quebre os relógios das celas: {r}"));
    }
    if let Some(r) = text.strip_prefix("Flying books ") {
        return Cow::Owned(format!("Livros voadores {r}"));
    }
    if let Some(r) = text.strip_prefix("God mode ") {
        return Cow::Owned(format!("God mode {r}"));
    }
    if let Some(r) = text.strip_prefix("Game saved to ") {
        return Cow::Owned(format!("Jogo salvo em {r}"));
    }
    if let Some(r) = text.strip_prefix("Save failed: ") {
        return Cow::Owned(format!("Falha ao salvar: {r}"));
    }
    for (en, pt) in [
        (" or click to resume", " ou clique para continuar"),
        ("  next line", "  próxima fala"),
        (" to talk", " para conversar"),
        (" talk again", " falar de novo"),
        (" Talk again", " Falar de novo"),
        (" to Talk", " para conversar"),
        (" Talk", " Falar"),
        (" Sanity", " Sanidade"),
        (": chapters", ": capítulos"),
    ] {
        if let Some(k) = text.strip_suffix(en) {
            return Cow::Owned(format!("{k}{pt}"));
        }
    }
    for (en, pt) in PREFIX {
        if let Some(rest) = text.strip_prefix(en) {
            return Cow::Owned(format!("{pt}{}", tr(rest)));
        }
    }
    for (en, pt) in SUFFIX {
        if let Some(head) = text.strip_suffix(en) {
            return Cow::Owned(format!("{head}{pt}"));
        }
    }
    // "Rage Box  12s" -> traduz o nome antes do tempo.
    if let Some((head, tail)) = text.rsplit_once("  ") {
        if let Cow::Borrowed(pt) = tr(head) {
            if pt != head { return Cow::Owned(format!("{pt}  {tail}")); }
        }
    }
    let mut out = Cow::Borrowed(text);
    for (en, pt) in PART {
        if out.contains(en) {
            out = Cow::Owned(out.replace(en, pt));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::tr;
    #[test]
    fn traducoes_basicas() {
        assert_eq!(tr("Collected Vorpal Blade"), "Coletado: Lâmina Vorpal");
        assert_eq!(tr("Rage Box  12s"), "Caixa da Fúria  12s");
        assert_eq!(tr("Sanity 85    Will 95    Primary cost: 3"),
                   "Sanidade 85    Vontade 95    Custo primário: 3");
        assert_eq!(tr("3 dice"), "3 dados");
        assert_eq!(tr("Texto desconhecido"), "Texto desconhecido");
    }
    #[test]
    fn traduz_exato_prefixo_e_partes() {
        assert_eq!(tr("Apply"), "Aplicar");
        assert_eq!(tr("FPS Limit: 60"), "Limite de FPS: 60");
        assert_eq!(tr("Pool of Tears - return"), "Lago de Lágrimas - retorno");
        assert_eq!(tr("E  next line"), "E  próxima fala");
        assert_eq!(tr("Aplicar"), "Aplicar");
        assert_eq!(tr("Hold Enter to skip scene"), "Segure Enter para pular a cena");
        assert_eq!(tr("Scene skipped"), "Cena pulada");
    }
}
static PHRASES: &[(&str, &str)] = &[
    ("Talk again to ", "Falar de novo com "),
    ("Talk to ", "Falar com "),
    ("Gnome", "Gnomo"),
    ("Forward or Space pull up", "Frente ou Espaço: subir"),
    (" pull up", ": subir"),
    (" move along edge", ": mover pela borda"),
    ("Back or Ctrl let go", "Trás ou Ctrl: soltar"),
    (" let go", ": soltar"),
    (" release rope", ": soltar corda"),
    ("Space up", "Espaço: subir"),
    ("Ctrl down", "Ctrl: descer"),
    (" swing", ": balançar"),
    (" to load", " para carregar"),
    (" to save", " para salvar"),
];

pub fn tr(text: &str) -> Cow<'_, str> {
    // AUTO-EXATO-INICIO: a frase inteira tem prioridade
    if let Some(&(_, pt)) = EXACT.iter().find(|(en, _)| *en == text) {
        return Cow::Borrowed(pt);
    }
    // AUTO-EXATO-FIM
    let base = tr_base(text);
    if base != text {
        return base;
    }
    let mut out = text.to_string();
    for (en, pt) in PHRASES {
        if out.contains(en) {
            out = out.replace(en, pt);
        }
    }
    if out == text { Cow::Borrowed(text) } else { Cow::Owned(out) }
}
