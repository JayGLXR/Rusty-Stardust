SECTIONS
{
    .text :
    {
        *( .text$A );
        *( .text$B );
        *( .rdata* );
        *( .text$C );
    }

    /* Discard exception handling and debug sections */
    /DISCARD/ :
    {
        *(.pdata);
        *(.xdata);
        *(.debug*);
        *(.eh_frame);
        *(.note*);
        *(.comment*);
    }
}