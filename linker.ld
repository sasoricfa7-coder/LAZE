OUTPUT_FORMAT("binary") /* Sortie brute directe */
ENTRY(_start)

SECTIONS {
    . = 0x7c00;

    .text : {
        *(.text._start) /* Notre point d'entrée en premier */
        *(.text .text.*)
        *(.rodata .rodata.*)
        *(.data .data.*)
        
        /* On remplit avec des zéros jusqu'à l'octet 510 du secteur */
        . = 0x7c00 + 510;
        
        /* Signature magique de boot MBR */
        BYTE(0x55);
        BYTE(0xAA);
    }

    /DISCARD/ : {
        *(.bss .bss.*) *(.eh_frame) *(.note .gnu.property) *(.comment)
    }
}
