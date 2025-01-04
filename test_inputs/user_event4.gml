switch tmu_state {
    
    case 0: // TMU_OPENING
        break;
    
    case 1: // TMU_ITEM
    case 2: // TMU_ITEM_CLOSING
        var loc = tmu_display_row*columns;
        var draw_row = 0;
        var draw_column = 0;
        var endpoint = array_length(tmu_item_panel_contents);
        var show_add = false;
        break;
    
}