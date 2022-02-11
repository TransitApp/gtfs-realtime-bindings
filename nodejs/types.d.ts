import * as $protobuf from "protobufjs";
/** Properties of a TransitAlertExtension. */
export interface ITransitAlertExtension {

    /** TransitAlertExtension createdAt */
    createdAt: (number|Long);

    /** TransitAlertExtension sourceName */
    sourceName?: (string|null);

    /** TransitAlertExtension isServiceChangeAlert */
    isServiceChangeAlert?: (boolean|null);
}

/** Represents a TransitAlertExtension. */
export class TransitAlertExtension implements ITransitAlertExtension {

    /**
     * Constructs a new TransitAlertExtension.
     * @param [properties] Properties to set
     */
    constructor(properties?: ITransitAlertExtension);

    /** TransitAlertExtension createdAt. */
    public createdAt: (number|Long);

    /** TransitAlertExtension sourceName. */
    public sourceName: string;

    /** TransitAlertExtension isServiceChangeAlert. */
    public isServiceChangeAlert: boolean;

    /**
     * Creates a new TransitAlertExtension instance using the specified properties.
     * @param [properties] Properties to set
     * @returns TransitAlertExtension instance
     */
    public static create(properties?: ITransitAlertExtension): TransitAlertExtension;

    /**
     * Encodes the specified TransitAlertExtension message. Does not implicitly {@link TransitAlertExtension.verify|verify} messages.
     * @param message TransitAlertExtension message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encode(message: ITransitAlertExtension, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Encodes the specified TransitAlertExtension message, length delimited. Does not implicitly {@link TransitAlertExtension.verify|verify} messages.
     * @param message TransitAlertExtension message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encodeDelimited(message: ITransitAlertExtension, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Decodes a TransitAlertExtension message from the specified reader or buffer.
     * @param reader Reader or buffer to decode from
     * @param [length] Message length if known beforehand
     * @returns TransitAlertExtension
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): TransitAlertExtension;

    /**
     * Decodes a TransitAlertExtension message from the specified reader or buffer, length delimited.
     * @param reader Reader or buffer to decode from
     * @returns TransitAlertExtension
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): TransitAlertExtension;

    /**
     * Verifies a TransitAlertExtension message.
     * @param message Plain object to verify
     * @returns `null` if valid, otherwise the reason why it is not
     */
    public static verify(message: { [k: string]: any }): (string|null);

    /**
     * Creates a TransitAlertExtension message from a plain object. Also converts values to their respective internal types.
     * @param object Plain object
     * @returns TransitAlertExtension
     */
    public static fromObject(object: { [k: string]: any }): TransitAlertExtension;

    /**
     * Creates a plain object from a TransitAlertExtension message. Also converts values to other types if specified.
     * @param message TransitAlertExtension
     * @param [options] Conversion options
     * @returns Plain object
     */
    public static toObject(message: TransitAlertExtension, options?: $protobuf.IConversionOptions): { [k: string]: any };

    /**
     * Converts this TransitAlertExtension to JSON.
     * @returns JSON object
     */
    public toJSON(): { [k: string]: any };
}

/** Properties of a TransitInformedEntityExtension. */
export interface ITransitInformedEntityExtension {

    /** TransitInformedEntityExtension feedId */
    feedId?: (number|null);

    /** TransitInformedEntityExtension globalRouteId */
    globalRouteId?: (number|null);

    /** TransitInformedEntityExtension stableStopId */
    stableStopId?: (number|null);
}

/** Represents a TransitInformedEntityExtension. */
export class TransitInformedEntityExtension implements ITransitInformedEntityExtension {

    /**
     * Constructs a new TransitInformedEntityExtension.
     * @param [properties] Properties to set
     */
    constructor(properties?: ITransitInformedEntityExtension);

    /** TransitInformedEntityExtension feedId. */
    public feedId: number;

    /** TransitInformedEntityExtension globalRouteId. */
    public globalRouteId: number;

    /** TransitInformedEntityExtension stableStopId. */
    public stableStopId: number;

    /**
     * Creates a new TransitInformedEntityExtension instance using the specified properties.
     * @param [properties] Properties to set
     * @returns TransitInformedEntityExtension instance
     */
    public static create(properties?: ITransitInformedEntityExtension): TransitInformedEntityExtension;

    /**
     * Encodes the specified TransitInformedEntityExtension message. Does not implicitly {@link TransitInformedEntityExtension.verify|verify} messages.
     * @param message TransitInformedEntityExtension message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encode(message: ITransitInformedEntityExtension, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Encodes the specified TransitInformedEntityExtension message, length delimited. Does not implicitly {@link TransitInformedEntityExtension.verify|verify} messages.
     * @param message TransitInformedEntityExtension message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encodeDelimited(message: ITransitInformedEntityExtension, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Decodes a TransitInformedEntityExtension message from the specified reader or buffer.
     * @param reader Reader or buffer to decode from
     * @param [length] Message length if known beforehand
     * @returns TransitInformedEntityExtension
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): TransitInformedEntityExtension;

    /**
     * Decodes a TransitInformedEntityExtension message from the specified reader or buffer, length delimited.
     * @param reader Reader or buffer to decode from
     * @returns TransitInformedEntityExtension
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): TransitInformedEntityExtension;

    /**
     * Verifies a TransitInformedEntityExtension message.
     * @param message Plain object to verify
     * @returns `null` if valid, otherwise the reason why it is not
     */
    public static verify(message: { [k: string]: any }): (string|null);

    /**
     * Creates a TransitInformedEntityExtension message from a plain object. Also converts values to their respective internal types.
     * @param object Plain object
     * @returns TransitInformedEntityExtension
     */
    public static fromObject(object: { [k: string]: any }): TransitInformedEntityExtension;

    /**
     * Creates a plain object from a TransitInformedEntityExtension message. Also converts values to other types if specified.
     * @param message TransitInformedEntityExtension
     * @param [options] Conversion options
     * @returns Plain object
     */
    public static toObject(message: TransitInformedEntityExtension, options?: $protobuf.IConversionOptions): { [k: string]: any };

    /**
     * Converts this TransitInformedEntityExtension to JSON.
     * @returns JSON object
     */
    public toJSON(): { [k: string]: any };
}

/** Properties of a TransitVehicleDescriptorExtension. */
export interface ITransitVehicleDescriptorExtension {

    /** TransitVehicleDescriptorExtension basedOnCrowdsourcingData */
    basedOnCrowdsourcingData?: (boolean|null);

    /** TransitVehicleDescriptorExtension transitVehicleId */
    transitVehicleId?: (string|null);
}

/** Represents a TransitVehicleDescriptorExtension. */
export class TransitVehicleDescriptorExtension implements ITransitVehicleDescriptorExtension {

    /**
     * Constructs a new TransitVehicleDescriptorExtension.
     * @param [properties] Properties to set
     */
    constructor(properties?: ITransitVehicleDescriptorExtension);

    /** TransitVehicleDescriptorExtension basedOnCrowdsourcingData. */
    public basedOnCrowdsourcingData: boolean;

    /** TransitVehicleDescriptorExtension transitVehicleId. */
    public transitVehicleId: string;

    /**
     * Creates a new TransitVehicleDescriptorExtension instance using the specified properties.
     * @param [properties] Properties to set
     * @returns TransitVehicleDescriptorExtension instance
     */
    public static create(properties?: ITransitVehicleDescriptorExtension): TransitVehicleDescriptorExtension;

    /**
     * Encodes the specified TransitVehicleDescriptorExtension message. Does not implicitly {@link TransitVehicleDescriptorExtension.verify|verify} messages.
     * @param message TransitVehicleDescriptorExtension message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encode(message: ITransitVehicleDescriptorExtension, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Encodes the specified TransitVehicleDescriptorExtension message, length delimited. Does not implicitly {@link TransitVehicleDescriptorExtension.verify|verify} messages.
     * @param message TransitVehicleDescriptorExtension message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encodeDelimited(message: ITransitVehicleDescriptorExtension, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Decodes a TransitVehicleDescriptorExtension message from the specified reader or buffer.
     * @param reader Reader or buffer to decode from
     * @param [length] Message length if known beforehand
     * @returns TransitVehicleDescriptorExtension
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): TransitVehicleDescriptorExtension;

    /**
     * Decodes a TransitVehicleDescriptorExtension message from the specified reader or buffer, length delimited.
     * @param reader Reader or buffer to decode from
     * @returns TransitVehicleDescriptorExtension
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): TransitVehicleDescriptorExtension;

    /**
     * Verifies a TransitVehicleDescriptorExtension message.
     * @param message Plain object to verify
     * @returns `null` if valid, otherwise the reason why it is not
     */
    public static verify(message: { [k: string]: any }): (string|null);

    /**
     * Creates a TransitVehicleDescriptorExtension message from a plain object. Also converts values to their respective internal types.
     * @param object Plain object
     * @returns TransitVehicleDescriptorExtension
     */
    public static fromObject(object: { [k: string]: any }): TransitVehicleDescriptorExtension;

    /**
     * Creates a plain object from a TransitVehicleDescriptorExtension message. Also converts values to other types if specified.
     * @param message TransitVehicleDescriptorExtension
     * @param [options] Conversion options
     * @returns Plain object
     */
    public static toObject(message: TransitVehicleDescriptorExtension, options?: $protobuf.IConversionOptions): { [k: string]: any };

    /**
     * Converts this TransitVehicleDescriptorExtension to JSON.
     * @returns JSON object
     */
    public toJSON(): { [k: string]: any };
}

/** Properties of a TransitTripDescriptorExtension. */
export interface ITransitTripDescriptorExtension {

    /** TransitTripDescriptorExtension originPrediction */
    originPrediction?: (TransitTripDescriptorExtension.OriginPrediction|null);

    /** TransitTripDescriptorExtension predictionMadeAt */
    predictionMadeAt?: (number|Long|null);
}

/** Represents a TransitTripDescriptorExtension. */
export class TransitTripDescriptorExtension implements ITransitTripDescriptorExtension {

    /**
     * Constructs a new TransitTripDescriptorExtension.
     * @param [properties] Properties to set
     */
    constructor(properties?: ITransitTripDescriptorExtension);

    /** TransitTripDescriptorExtension originPrediction. */
    public originPrediction: TransitTripDescriptorExtension.OriginPrediction;

    /** TransitTripDescriptorExtension predictionMadeAt. */
    public predictionMadeAt: (number|Long);

    /**
     * Creates a new TransitTripDescriptorExtension instance using the specified properties.
     * @param [properties] Properties to set
     * @returns TransitTripDescriptorExtension instance
     */
    public static create(properties?: ITransitTripDescriptorExtension): TransitTripDescriptorExtension;

    /**
     * Encodes the specified TransitTripDescriptorExtension message. Does not implicitly {@link TransitTripDescriptorExtension.verify|verify} messages.
     * @param message TransitTripDescriptorExtension message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encode(message: ITransitTripDescriptorExtension, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Encodes the specified TransitTripDescriptorExtension message, length delimited. Does not implicitly {@link TransitTripDescriptorExtension.verify|verify} messages.
     * @param message TransitTripDescriptorExtension message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encodeDelimited(message: ITransitTripDescriptorExtension, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Decodes a TransitTripDescriptorExtension message from the specified reader or buffer.
     * @param reader Reader or buffer to decode from
     * @param [length] Message length if known beforehand
     * @returns TransitTripDescriptorExtension
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): TransitTripDescriptorExtension;

    /**
     * Decodes a TransitTripDescriptorExtension message from the specified reader or buffer, length delimited.
     * @param reader Reader or buffer to decode from
     * @returns TransitTripDescriptorExtension
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): TransitTripDescriptorExtension;

    /**
     * Verifies a TransitTripDescriptorExtension message.
     * @param message Plain object to verify
     * @returns `null` if valid, otherwise the reason why it is not
     */
    public static verify(message: { [k: string]: any }): (string|null);

    /**
     * Creates a TransitTripDescriptorExtension message from a plain object. Also converts values to their respective internal types.
     * @param object Plain object
     * @returns TransitTripDescriptorExtension
     */
    public static fromObject(object: { [k: string]: any }): TransitTripDescriptorExtension;

    /**
     * Creates a plain object from a TransitTripDescriptorExtension message. Also converts values to other types if specified.
     * @param message TransitTripDescriptorExtension
     * @param [options] Conversion options
     * @returns Plain object
     */
    public static toObject(message: TransitTripDescriptorExtension, options?: $protobuf.IConversionOptions): { [k: string]: any };

    /**
     * Converts this TransitTripDescriptorExtension to JSON.
     * @returns JSON object
     */
    public toJSON(): { [k: string]: any };
}

export namespace TransitTripDescriptorExtension {

    /** OriginPrediction enum. */
    enum OriginPrediction {
        AGENCY_ORIGINAL_DATA = 0,
        TRANSIT_PREDICTION_ENGINE = 1
    }
}

/** Properties of a TransitStopTimeUpdateExtension. */
export interface ITransitStopTimeUpdateExtension {

    /** TransitStopTimeUpdateExtension shouldNotAssumeVehicleIsPast */
    shouldNotAssumeVehicleIsPast?: (boolean|null);

    /** TransitStopTimeUpdateExtension predictionAlgorithm */
    predictionAlgorithm?: (TransitStopTimeUpdateExtension.PredictionAlgorithm|null);
}

/** Represents a TransitStopTimeUpdateExtension. */
export class TransitStopTimeUpdateExtension implements ITransitStopTimeUpdateExtension {

    /**
     * Constructs a new TransitStopTimeUpdateExtension.
     * @param [properties] Properties to set
     */
    constructor(properties?: ITransitStopTimeUpdateExtension);

    /** TransitStopTimeUpdateExtension shouldNotAssumeVehicleIsPast. */
    public shouldNotAssumeVehicleIsPast: boolean;

    /** TransitStopTimeUpdateExtension predictionAlgorithm. */
    public predictionAlgorithm: TransitStopTimeUpdateExtension.PredictionAlgorithm;

    /**
     * Creates a new TransitStopTimeUpdateExtension instance using the specified properties.
     * @param [properties] Properties to set
     * @returns TransitStopTimeUpdateExtension instance
     */
    public static create(properties?: ITransitStopTimeUpdateExtension): TransitStopTimeUpdateExtension;

    /**
     * Encodes the specified TransitStopTimeUpdateExtension message. Does not implicitly {@link TransitStopTimeUpdateExtension.verify|verify} messages.
     * @param message TransitStopTimeUpdateExtension message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encode(message: ITransitStopTimeUpdateExtension, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Encodes the specified TransitStopTimeUpdateExtension message, length delimited. Does not implicitly {@link TransitStopTimeUpdateExtension.verify|verify} messages.
     * @param message TransitStopTimeUpdateExtension message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encodeDelimited(message: ITransitStopTimeUpdateExtension, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Decodes a TransitStopTimeUpdateExtension message from the specified reader or buffer.
     * @param reader Reader or buffer to decode from
     * @param [length] Message length if known beforehand
     * @returns TransitStopTimeUpdateExtension
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): TransitStopTimeUpdateExtension;

    /**
     * Decodes a TransitStopTimeUpdateExtension message from the specified reader or buffer, length delimited.
     * @param reader Reader or buffer to decode from
     * @returns TransitStopTimeUpdateExtension
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): TransitStopTimeUpdateExtension;

    /**
     * Verifies a TransitStopTimeUpdateExtension message.
     * @param message Plain object to verify
     * @returns `null` if valid, otherwise the reason why it is not
     */
    public static verify(message: { [k: string]: any }): (string|null);

    /**
     * Creates a TransitStopTimeUpdateExtension message from a plain object. Also converts values to their respective internal types.
     * @param object Plain object
     * @returns TransitStopTimeUpdateExtension
     */
    public static fromObject(object: { [k: string]: any }): TransitStopTimeUpdateExtension;

    /**
     * Creates a plain object from a TransitStopTimeUpdateExtension message. Also converts values to other types if specified.
     * @param message TransitStopTimeUpdateExtension
     * @param [options] Conversion options
     * @returns Plain object
     */
    public static toObject(message: TransitStopTimeUpdateExtension, options?: $protobuf.IConversionOptions): { [k: string]: any };

    /**
     * Converts this TransitStopTimeUpdateExtension to JSON.
     * @returns JSON object
     */
    public toJSON(): { [k: string]: any };
}

export namespace TransitStopTimeUpdateExtension {

    /** PredictionAlgorithm enum. */
    enum PredictionAlgorithm {
        ML = 0,
        RECENCY = 1,
        DETERMINISTIC = 2,
        PROPAGATION = 3
    }
}

/** Namespace transit_realtime. */
export namespace transit_realtime {

    /** Properties of a FeedMessage. */
    interface IFeedMessage {

        /** FeedMessage header */
        header: transit_realtime.IFeedHeader;

        /** FeedMessage entity */
        entity?: (transit_realtime.IFeedEntity[]|null);
    }

    /** Represents a FeedMessage. */
    class FeedMessage implements IFeedMessage {

        /**
         * Constructs a new FeedMessage.
         * @param [properties] Properties to set
         */
        constructor(properties?: transit_realtime.IFeedMessage);

        /** FeedMessage header. */
        public header: transit_realtime.IFeedHeader;

        /** FeedMessage entity. */
        public entity: transit_realtime.IFeedEntity[];

        /**
         * Creates a new FeedMessage instance using the specified properties.
         * @param [properties] Properties to set
         * @returns FeedMessage instance
         */
        public static create(properties?: transit_realtime.IFeedMessage): transit_realtime.FeedMessage;

        /**
         * Encodes the specified FeedMessage message. Does not implicitly {@link transit_realtime.FeedMessage.verify|verify} messages.
         * @param message FeedMessage message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encode(message: transit_realtime.IFeedMessage, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Encodes the specified FeedMessage message, length delimited. Does not implicitly {@link transit_realtime.FeedMessage.verify|verify} messages.
         * @param message FeedMessage message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encodeDelimited(message: transit_realtime.IFeedMessage, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Decodes a FeedMessage message from the specified reader or buffer.
         * @param reader Reader or buffer to decode from
         * @param [length] Message length if known beforehand
         * @returns FeedMessage
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): transit_realtime.FeedMessage;

        /**
         * Decodes a FeedMessage message from the specified reader or buffer, length delimited.
         * @param reader Reader or buffer to decode from
         * @returns FeedMessage
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): transit_realtime.FeedMessage;

        /**
         * Verifies a FeedMessage message.
         * @param message Plain object to verify
         * @returns `null` if valid, otherwise the reason why it is not
         */
        public static verify(message: { [k: string]: any }): (string|null);

        /**
         * Creates a FeedMessage message from a plain object. Also converts values to their respective internal types.
         * @param object Plain object
         * @returns FeedMessage
         */
        public static fromObject(object: { [k: string]: any }): transit_realtime.FeedMessage;

        /**
         * Creates a plain object from a FeedMessage message. Also converts values to other types if specified.
         * @param message FeedMessage
         * @param [options] Conversion options
         * @returns Plain object
         */
        public static toObject(message: transit_realtime.FeedMessage, options?: $protobuf.IConversionOptions): { [k: string]: any };

        /**
         * Converts this FeedMessage to JSON.
         * @returns JSON object
         */
        public toJSON(): { [k: string]: any };
    }

    /** Properties of a FeedHeader. */
    interface IFeedHeader {

        /** FeedHeader gtfsRealtimeVersion */
        gtfsRealtimeVersion: string;

        /** FeedHeader incrementality */
        incrementality?: (transit_realtime.FeedHeader.Incrementality|null);

        /** FeedHeader timestamp */
        timestamp?: (number|Long|null);

        /** FeedHeader .nyctFeedHeader */
        ".nyctFeedHeader"?: (INyctFeedHeader|null);
    }

    /** Represents a FeedHeader. */
    class FeedHeader implements IFeedHeader {

        /**
         * Constructs a new FeedHeader.
         * @param [properties] Properties to set
         */
        constructor(properties?: transit_realtime.IFeedHeader);

        /** FeedHeader gtfsRealtimeVersion. */
        public gtfsRealtimeVersion: string;

        /** FeedHeader incrementality. */
        public incrementality: transit_realtime.FeedHeader.Incrementality;

        /** FeedHeader timestamp. */
        public timestamp: (number|Long);

        /** FeedHeader .nyctFeedHeader. */
        public nyctFeedHeader?: (INyctFeedHeader|null);

        /**
         * Creates a new FeedHeader instance using the specified properties.
         * @param [properties] Properties to set
         * @returns FeedHeader instance
         */
        public static create(properties?: transit_realtime.IFeedHeader): transit_realtime.FeedHeader;

        /**
         * Encodes the specified FeedHeader message. Does not implicitly {@link transit_realtime.FeedHeader.verify|verify} messages.
         * @param message FeedHeader message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encode(message: transit_realtime.IFeedHeader, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Encodes the specified FeedHeader message, length delimited. Does not implicitly {@link transit_realtime.FeedHeader.verify|verify} messages.
         * @param message FeedHeader message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encodeDelimited(message: transit_realtime.IFeedHeader, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Decodes a FeedHeader message from the specified reader or buffer.
         * @param reader Reader or buffer to decode from
         * @param [length] Message length if known beforehand
         * @returns FeedHeader
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): transit_realtime.FeedHeader;

        /**
         * Decodes a FeedHeader message from the specified reader or buffer, length delimited.
         * @param reader Reader or buffer to decode from
         * @returns FeedHeader
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): transit_realtime.FeedHeader;

        /**
         * Verifies a FeedHeader message.
         * @param message Plain object to verify
         * @returns `null` if valid, otherwise the reason why it is not
         */
        public static verify(message: { [k: string]: any }): (string|null);

        /**
         * Creates a FeedHeader message from a plain object. Also converts values to their respective internal types.
         * @param object Plain object
         * @returns FeedHeader
         */
        public static fromObject(object: { [k: string]: any }): transit_realtime.FeedHeader;

        /**
         * Creates a plain object from a FeedHeader message. Also converts values to other types if specified.
         * @param message FeedHeader
         * @param [options] Conversion options
         * @returns Plain object
         */
        public static toObject(message: transit_realtime.FeedHeader, options?: $protobuf.IConversionOptions): { [k: string]: any };

        /**
         * Converts this FeedHeader to JSON.
         * @returns JSON object
         */
        public toJSON(): { [k: string]: any };
    }

    namespace FeedHeader {

        /** Incrementality enum. */
        enum Incrementality {
            FULL_DATASET = 0,
            DIFFERENTIAL = 1
        }
    }

    /** Properties of a FeedEntity. */
    interface IFeedEntity {

        /** FeedEntity id */
        id: string;

        /** FeedEntity isDeleted */
        isDeleted?: (boolean|null);

        /** FeedEntity tripUpdate */
        tripUpdate?: (transit_realtime.ITripUpdate|null);

        /** FeedEntity vehicle */
        vehicle?: (transit_realtime.IVehiclePosition|null);

        /** FeedEntity alert */
        alert?: (transit_realtime.IAlert|null);

        /** FeedEntity shape */
        shape?: (transit_realtime.IShape|null);
    }

    /** Represents a FeedEntity. */
    class FeedEntity implements IFeedEntity {

        /**
         * Constructs a new FeedEntity.
         * @param [properties] Properties to set
         */
        constructor(properties?: transit_realtime.IFeedEntity);

        /** FeedEntity id. */
        public id: string;

        /** FeedEntity isDeleted. */
        public isDeleted: boolean;

        /** FeedEntity tripUpdate. */
        public tripUpdate?: (transit_realtime.ITripUpdate|null);

        /** FeedEntity vehicle. */
        public vehicle?: (transit_realtime.IVehiclePosition|null);

        /** FeedEntity alert. */
        public alert?: (transit_realtime.IAlert|null);

        /** FeedEntity shape. */
        public shape?: (transit_realtime.IShape|null);

        /**
         * Creates a new FeedEntity instance using the specified properties.
         * @param [properties] Properties to set
         * @returns FeedEntity instance
         */
        public static create(properties?: transit_realtime.IFeedEntity): transit_realtime.FeedEntity;

        /**
         * Encodes the specified FeedEntity message. Does not implicitly {@link transit_realtime.FeedEntity.verify|verify} messages.
         * @param message FeedEntity message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encode(message: transit_realtime.IFeedEntity, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Encodes the specified FeedEntity message, length delimited. Does not implicitly {@link transit_realtime.FeedEntity.verify|verify} messages.
         * @param message FeedEntity message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encodeDelimited(message: transit_realtime.IFeedEntity, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Decodes a FeedEntity message from the specified reader or buffer.
         * @param reader Reader or buffer to decode from
         * @param [length] Message length if known beforehand
         * @returns FeedEntity
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): transit_realtime.FeedEntity;

        /**
         * Decodes a FeedEntity message from the specified reader or buffer, length delimited.
         * @param reader Reader or buffer to decode from
         * @returns FeedEntity
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): transit_realtime.FeedEntity;

        /**
         * Verifies a FeedEntity message.
         * @param message Plain object to verify
         * @returns `null` if valid, otherwise the reason why it is not
         */
        public static verify(message: { [k: string]: any }): (string|null);

        /**
         * Creates a FeedEntity message from a plain object. Also converts values to their respective internal types.
         * @param object Plain object
         * @returns FeedEntity
         */
        public static fromObject(object: { [k: string]: any }): transit_realtime.FeedEntity;

        /**
         * Creates a plain object from a FeedEntity message. Also converts values to other types if specified.
         * @param message FeedEntity
         * @param [options] Conversion options
         * @returns Plain object
         */
        public static toObject(message: transit_realtime.FeedEntity, options?: $protobuf.IConversionOptions): { [k: string]: any };

        /**
         * Converts this FeedEntity to JSON.
         * @returns JSON object
         */
        public toJSON(): { [k: string]: any };
    }

    /** Properties of a TripUpdate. */
    interface ITripUpdate {

        /** TripUpdate trip */
        trip: transit_realtime.ITripDescriptor;

        /** TripUpdate vehicle */
        vehicle?: (transit_realtime.IVehicleDescriptor|null);

        /** TripUpdate stopTimeUpdate */
        stopTimeUpdate?: (transit_realtime.TripUpdate.IStopTimeUpdate[]|null);

        /** TripUpdate timestamp */
        timestamp?: (number|Long|null);

        /** TripUpdate delay */
        delay?: (number|null);

        /** TripUpdate tripProperties */
        tripProperties?: (transit_realtime.TripUpdate.ITripProperties|null);
    }

    /** Represents a TripUpdate. */
    class TripUpdate implements ITripUpdate {

        /**
         * Constructs a new TripUpdate.
         * @param [properties] Properties to set
         */
        constructor(properties?: transit_realtime.ITripUpdate);

        /** TripUpdate trip. */
        public trip: transit_realtime.ITripDescriptor;

        /** TripUpdate vehicle. */
        public vehicle?: (transit_realtime.IVehicleDescriptor|null);

        /** TripUpdate stopTimeUpdate. */
        public stopTimeUpdate: transit_realtime.TripUpdate.IStopTimeUpdate[];

        /** TripUpdate timestamp. */
        public timestamp: (number|Long);

        /** TripUpdate delay. */
        public delay: number;

        /** TripUpdate tripProperties. */
        public tripProperties?: (transit_realtime.TripUpdate.ITripProperties|null);

        /**
         * Creates a new TripUpdate instance using the specified properties.
         * @param [properties] Properties to set
         * @returns TripUpdate instance
         */
        public static create(properties?: transit_realtime.ITripUpdate): transit_realtime.TripUpdate;

        /**
         * Encodes the specified TripUpdate message. Does not implicitly {@link transit_realtime.TripUpdate.verify|verify} messages.
         * @param message TripUpdate message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encode(message: transit_realtime.ITripUpdate, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Encodes the specified TripUpdate message, length delimited. Does not implicitly {@link transit_realtime.TripUpdate.verify|verify} messages.
         * @param message TripUpdate message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encodeDelimited(message: transit_realtime.ITripUpdate, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Decodes a TripUpdate message from the specified reader or buffer.
         * @param reader Reader or buffer to decode from
         * @param [length] Message length if known beforehand
         * @returns TripUpdate
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): transit_realtime.TripUpdate;

        /**
         * Decodes a TripUpdate message from the specified reader or buffer, length delimited.
         * @param reader Reader or buffer to decode from
         * @returns TripUpdate
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): transit_realtime.TripUpdate;

        /**
         * Verifies a TripUpdate message.
         * @param message Plain object to verify
         * @returns `null` if valid, otherwise the reason why it is not
         */
        public static verify(message: { [k: string]: any }): (string|null);

        /**
         * Creates a TripUpdate message from a plain object. Also converts values to their respective internal types.
         * @param object Plain object
         * @returns TripUpdate
         */
        public static fromObject(object: { [k: string]: any }): transit_realtime.TripUpdate;

        /**
         * Creates a plain object from a TripUpdate message. Also converts values to other types if specified.
         * @param message TripUpdate
         * @param [options] Conversion options
         * @returns Plain object
         */
        public static toObject(message: transit_realtime.TripUpdate, options?: $protobuf.IConversionOptions): { [k: string]: any };

        /**
         * Converts this TripUpdate to JSON.
         * @returns JSON object
         */
        public toJSON(): { [k: string]: any };
    }

    namespace TripUpdate {

        /** Properties of a StopTimeEvent. */
        interface IStopTimeEvent {

            /** StopTimeEvent delay */
            delay?: (number|null);

            /** StopTimeEvent time */
            time?: (number|Long|null);

            /** StopTimeEvent uncertainty */
            uncertainty?: (number|null);
        }

        /** Represents a StopTimeEvent. */
        class StopTimeEvent implements IStopTimeEvent {

            /**
             * Constructs a new StopTimeEvent.
             * @param [properties] Properties to set
             */
            constructor(properties?: transit_realtime.TripUpdate.IStopTimeEvent);

            /** StopTimeEvent delay. */
            public delay: number;

            /** StopTimeEvent time. */
            public time: (number|Long);

            /** StopTimeEvent uncertainty. */
            public uncertainty: number;

            /**
             * Creates a new StopTimeEvent instance using the specified properties.
             * @param [properties] Properties to set
             * @returns StopTimeEvent instance
             */
            public static create(properties?: transit_realtime.TripUpdate.IStopTimeEvent): transit_realtime.TripUpdate.StopTimeEvent;

            /**
             * Encodes the specified StopTimeEvent message. Does not implicitly {@link transit_realtime.TripUpdate.StopTimeEvent.verify|verify} messages.
             * @param message StopTimeEvent message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: transit_realtime.TripUpdate.IStopTimeEvent, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified StopTimeEvent message, length delimited. Does not implicitly {@link transit_realtime.TripUpdate.StopTimeEvent.verify|verify} messages.
             * @param message StopTimeEvent message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: transit_realtime.TripUpdate.IStopTimeEvent, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a StopTimeEvent message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns StopTimeEvent
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): transit_realtime.TripUpdate.StopTimeEvent;

            /**
             * Decodes a StopTimeEvent message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns StopTimeEvent
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): transit_realtime.TripUpdate.StopTimeEvent;

            /**
             * Verifies a StopTimeEvent message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a StopTimeEvent message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns StopTimeEvent
             */
            public static fromObject(object: { [k: string]: any }): transit_realtime.TripUpdate.StopTimeEvent;

            /**
             * Creates a plain object from a StopTimeEvent message. Also converts values to other types if specified.
             * @param message StopTimeEvent
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: transit_realtime.TripUpdate.StopTimeEvent, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this StopTimeEvent to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of a StopTimeUpdate. */
        interface IStopTimeUpdate {

            /** StopTimeUpdate stopSequence */
            stopSequence?: (number|null);

            /** StopTimeUpdate stopId */
            stopId?: (string|null);

            /** StopTimeUpdate arrival */
            arrival?: (transit_realtime.TripUpdate.IStopTimeEvent|null);

            /** StopTimeUpdate departure */
            departure?: (transit_realtime.TripUpdate.IStopTimeEvent|null);

            /** StopTimeUpdate departureOccupancyStatus */
            departureOccupancyStatus?: (transit_realtime.VehiclePosition.OccupancyStatus|null);

            /** StopTimeUpdate scheduleRelationship */
            scheduleRelationship?: (transit_realtime.TripUpdate.StopTimeUpdate.ScheduleRelationship|null);

            /** StopTimeUpdate stopTimeProperties */
            stopTimeProperties?: (transit_realtime.TripUpdate.StopTimeUpdate.IStopTimeProperties|null);

            /** StopTimeUpdate .transitStopTimeUpdateExtension */
            ".transitStopTimeUpdateExtension"?: (ITransitStopTimeUpdateExtension|null);

            /** StopTimeUpdate .nyctStopTimeUpdate */
            ".nyctStopTimeUpdate"?: (INyctStopTimeUpdate|null);
        }

        /** Represents a StopTimeUpdate. */
        class StopTimeUpdate implements IStopTimeUpdate {

            /**
             * Constructs a new StopTimeUpdate.
             * @param [properties] Properties to set
             */
            constructor(properties?: transit_realtime.TripUpdate.IStopTimeUpdate);

            /** StopTimeUpdate stopSequence. */
            public stopSequence: number;

            /** StopTimeUpdate stopId. */
            public stopId: string;

            /** StopTimeUpdate arrival. */
            public arrival?: (transit_realtime.TripUpdate.IStopTimeEvent|null);

            /** StopTimeUpdate departure. */
            public departure?: (transit_realtime.TripUpdate.IStopTimeEvent|null);

            /** StopTimeUpdate departureOccupancyStatus. */
            public departureOccupancyStatus: transit_realtime.VehiclePosition.OccupancyStatus;

            /** StopTimeUpdate scheduleRelationship. */
            public scheduleRelationship: transit_realtime.TripUpdate.StopTimeUpdate.ScheduleRelationship;

            /** StopTimeUpdate stopTimeProperties. */
            public stopTimeProperties?: (transit_realtime.TripUpdate.StopTimeUpdate.IStopTimeProperties|null);

            /** StopTimeUpdate .transitStopTimeUpdateExtension. */
            public transitStopTimeUpdateExtension?: (ITransitStopTimeUpdateExtension|null);

            /** StopTimeUpdate .nyctStopTimeUpdate. */
            public nyctStopTimeUpdate?: (INyctStopTimeUpdate|null);

            /**
             * Creates a new StopTimeUpdate instance using the specified properties.
             * @param [properties] Properties to set
             * @returns StopTimeUpdate instance
             */
            public static create(properties?: transit_realtime.TripUpdate.IStopTimeUpdate): transit_realtime.TripUpdate.StopTimeUpdate;

            /**
             * Encodes the specified StopTimeUpdate message. Does not implicitly {@link transit_realtime.TripUpdate.StopTimeUpdate.verify|verify} messages.
             * @param message StopTimeUpdate message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: transit_realtime.TripUpdate.IStopTimeUpdate, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified StopTimeUpdate message, length delimited. Does not implicitly {@link transit_realtime.TripUpdate.StopTimeUpdate.verify|verify} messages.
             * @param message StopTimeUpdate message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: transit_realtime.TripUpdate.IStopTimeUpdate, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a StopTimeUpdate message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns StopTimeUpdate
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): transit_realtime.TripUpdate.StopTimeUpdate;

            /**
             * Decodes a StopTimeUpdate message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns StopTimeUpdate
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): transit_realtime.TripUpdate.StopTimeUpdate;

            /**
             * Verifies a StopTimeUpdate message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a StopTimeUpdate message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns StopTimeUpdate
             */
            public static fromObject(object: { [k: string]: any }): transit_realtime.TripUpdate.StopTimeUpdate;

            /**
             * Creates a plain object from a StopTimeUpdate message. Also converts values to other types if specified.
             * @param message StopTimeUpdate
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: transit_realtime.TripUpdate.StopTimeUpdate, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this StopTimeUpdate to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        namespace StopTimeUpdate {

            /** ScheduleRelationship enum. */
            enum ScheduleRelationship {
                SCHEDULED = 0,
                SKIPPED = 1,
                NO_DATA = 2,
                UNSCHEDULED = 3
            }

            /** Properties of a StopTimeProperties. */
            interface IStopTimeProperties {

                /** StopTimeProperties assignedStopId */
                assignedStopId?: (string|null);
            }

            /** Represents a StopTimeProperties. */
            class StopTimeProperties implements IStopTimeProperties {

                /**
                 * Constructs a new StopTimeProperties.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: transit_realtime.TripUpdate.StopTimeUpdate.IStopTimeProperties);

                /** StopTimeProperties assignedStopId. */
                public assignedStopId: string;

                /**
                 * Creates a new StopTimeProperties instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns StopTimeProperties instance
                 */
                public static create(properties?: transit_realtime.TripUpdate.StopTimeUpdate.IStopTimeProperties): transit_realtime.TripUpdate.StopTimeUpdate.StopTimeProperties;

                /**
                 * Encodes the specified StopTimeProperties message. Does not implicitly {@link transit_realtime.TripUpdate.StopTimeUpdate.StopTimeProperties.verify|verify} messages.
                 * @param message StopTimeProperties message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: transit_realtime.TripUpdate.StopTimeUpdate.IStopTimeProperties, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified StopTimeProperties message, length delimited. Does not implicitly {@link transit_realtime.TripUpdate.StopTimeUpdate.StopTimeProperties.verify|verify} messages.
                 * @param message StopTimeProperties message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: transit_realtime.TripUpdate.StopTimeUpdate.IStopTimeProperties, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a StopTimeProperties message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns StopTimeProperties
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): transit_realtime.TripUpdate.StopTimeUpdate.StopTimeProperties;

                /**
                 * Decodes a StopTimeProperties message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns StopTimeProperties
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): transit_realtime.TripUpdate.StopTimeUpdate.StopTimeProperties;

                /**
                 * Verifies a StopTimeProperties message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a StopTimeProperties message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns StopTimeProperties
                 */
                public static fromObject(object: { [k: string]: any }): transit_realtime.TripUpdate.StopTimeUpdate.StopTimeProperties;

                /**
                 * Creates a plain object from a StopTimeProperties message. Also converts values to other types if specified.
                 * @param message StopTimeProperties
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: transit_realtime.TripUpdate.StopTimeUpdate.StopTimeProperties, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this StopTimeProperties to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };
            }
        }

        /** Properties of a TripProperties. */
        interface ITripProperties {

            /** TripProperties tripId */
            tripId?: (string|null);

            /** TripProperties startDate */
            startDate?: (string|null);

            /** TripProperties startTime */
            startTime?: (string|null);

            /** TripProperties shapeId */
            shapeId?: (string|null);
        }

        /** Represents a TripProperties. */
        class TripProperties implements ITripProperties {

            /**
             * Constructs a new TripProperties.
             * @param [properties] Properties to set
             */
            constructor(properties?: transit_realtime.TripUpdate.ITripProperties);

            /** TripProperties tripId. */
            public tripId: string;

            /** TripProperties startDate. */
            public startDate: string;

            /** TripProperties startTime. */
            public startTime: string;

            /** TripProperties shapeId. */
            public shapeId: string;

            /**
             * Creates a new TripProperties instance using the specified properties.
             * @param [properties] Properties to set
             * @returns TripProperties instance
             */
            public static create(properties?: transit_realtime.TripUpdate.ITripProperties): transit_realtime.TripUpdate.TripProperties;

            /**
             * Encodes the specified TripProperties message. Does not implicitly {@link transit_realtime.TripUpdate.TripProperties.verify|verify} messages.
             * @param message TripProperties message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: transit_realtime.TripUpdate.ITripProperties, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified TripProperties message, length delimited. Does not implicitly {@link transit_realtime.TripUpdate.TripProperties.verify|verify} messages.
             * @param message TripProperties message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: transit_realtime.TripUpdate.ITripProperties, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a TripProperties message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns TripProperties
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): transit_realtime.TripUpdate.TripProperties;

            /**
             * Decodes a TripProperties message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns TripProperties
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): transit_realtime.TripUpdate.TripProperties;

            /**
             * Verifies a TripProperties message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a TripProperties message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns TripProperties
             */
            public static fromObject(object: { [k: string]: any }): transit_realtime.TripUpdate.TripProperties;

            /**
             * Creates a plain object from a TripProperties message. Also converts values to other types if specified.
             * @param message TripProperties
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: transit_realtime.TripUpdate.TripProperties, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this TripProperties to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }
    }

    /** Properties of a VehiclePosition. */
    interface IVehiclePosition {

        /** VehiclePosition trip */
        trip?: (transit_realtime.ITripDescriptor|null);

        /** VehiclePosition vehicle */
        vehicle?: (transit_realtime.IVehicleDescriptor|null);

        /** VehiclePosition position */
        position?: (transit_realtime.IPosition|null);

        /** VehiclePosition currentStopSequence */
        currentStopSequence?: (number|null);

        /** VehiclePosition stopId */
        stopId?: (string|null);

        /** VehiclePosition currentStatus */
        currentStatus?: (transit_realtime.VehiclePosition.VehicleStopStatus|null);

        /** VehiclePosition timestamp */
        timestamp?: (number|Long|null);

        /** VehiclePosition congestionLevel */
        congestionLevel?: (transit_realtime.VehiclePosition.CongestionLevel|null);

        /** VehiclePosition occupancyStatus */
        occupancyStatus?: (transit_realtime.VehiclePosition.OccupancyStatus|null);

        /** VehiclePosition occupancyPercentage */
        occupancyPercentage?: (number|null);

        /** VehiclePosition multiCarriageDetails */
        multiCarriageDetails?: (transit_realtime.VehiclePosition.ICarriageDetails[]|null);
    }

    /** Represents a VehiclePosition. */
    class VehiclePosition implements IVehiclePosition {

        /**
         * Constructs a new VehiclePosition.
         * @param [properties] Properties to set
         */
        constructor(properties?: transit_realtime.IVehiclePosition);

        /** VehiclePosition trip. */
        public trip?: (transit_realtime.ITripDescriptor|null);

        /** VehiclePosition vehicle. */
        public vehicle?: (transit_realtime.IVehicleDescriptor|null);

        /** VehiclePosition position. */
        public position?: (transit_realtime.IPosition|null);

        /** VehiclePosition currentStopSequence. */
        public currentStopSequence: number;

        /** VehiclePosition stopId. */
        public stopId: string;

        /** VehiclePosition currentStatus. */
        public currentStatus: transit_realtime.VehiclePosition.VehicleStopStatus;

        /** VehiclePosition timestamp. */
        public timestamp: (number|Long);

        /** VehiclePosition congestionLevel. */
        public congestionLevel: transit_realtime.VehiclePosition.CongestionLevel;

        /** VehiclePosition occupancyStatus. */
        public occupancyStatus: transit_realtime.VehiclePosition.OccupancyStatus;

        /** VehiclePosition occupancyPercentage. */
        public occupancyPercentage: number;

        /** VehiclePosition multiCarriageDetails. */
        public multiCarriageDetails: transit_realtime.VehiclePosition.ICarriageDetails[];

        /**
         * Creates a new VehiclePosition instance using the specified properties.
         * @param [properties] Properties to set
         * @returns VehiclePosition instance
         */
        public static create(properties?: transit_realtime.IVehiclePosition): transit_realtime.VehiclePosition;

        /**
         * Encodes the specified VehiclePosition message. Does not implicitly {@link transit_realtime.VehiclePosition.verify|verify} messages.
         * @param message VehiclePosition message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encode(message: transit_realtime.IVehiclePosition, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Encodes the specified VehiclePosition message, length delimited. Does not implicitly {@link transit_realtime.VehiclePosition.verify|verify} messages.
         * @param message VehiclePosition message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encodeDelimited(message: transit_realtime.IVehiclePosition, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Decodes a VehiclePosition message from the specified reader or buffer.
         * @param reader Reader or buffer to decode from
         * @param [length] Message length if known beforehand
         * @returns VehiclePosition
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): transit_realtime.VehiclePosition;

        /**
         * Decodes a VehiclePosition message from the specified reader or buffer, length delimited.
         * @param reader Reader or buffer to decode from
         * @returns VehiclePosition
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): transit_realtime.VehiclePosition;

        /**
         * Verifies a VehiclePosition message.
         * @param message Plain object to verify
         * @returns `null` if valid, otherwise the reason why it is not
         */
        public static verify(message: { [k: string]: any }): (string|null);

        /**
         * Creates a VehiclePosition message from a plain object. Also converts values to their respective internal types.
         * @param object Plain object
         * @returns VehiclePosition
         */
        public static fromObject(object: { [k: string]: any }): transit_realtime.VehiclePosition;

        /**
         * Creates a plain object from a VehiclePosition message. Also converts values to other types if specified.
         * @param message VehiclePosition
         * @param [options] Conversion options
         * @returns Plain object
         */
        public static toObject(message: transit_realtime.VehiclePosition, options?: $protobuf.IConversionOptions): { [k: string]: any };

        /**
         * Converts this VehiclePosition to JSON.
         * @returns JSON object
         */
        public toJSON(): { [k: string]: any };
    }

    namespace VehiclePosition {

        /** VehicleStopStatus enum. */
        enum VehicleStopStatus {
            INCOMING_AT = 0,
            STOPPED_AT = 1,
            IN_TRANSIT_TO = 2
        }

        /** CongestionLevel enum. */
        enum CongestionLevel {
            UNKNOWN_CONGESTION_LEVEL = 0,
            RUNNING_SMOOTHLY = 1,
            STOP_AND_GO = 2,
            CONGESTION = 3,
            SEVERE_CONGESTION = 4
        }

        /** OccupancyStatus enum. */
        enum OccupancyStatus {
            EMPTY = 0,
            MANY_SEATS_AVAILABLE = 1,
            FEW_SEATS_AVAILABLE = 2,
            STANDING_ROOM_ONLY = 3,
            CRUSHED_STANDING_ROOM_ONLY = 4,
            FULL = 5,
            NOT_ACCEPTING_PASSENGERS = 6,
            NO_DATA_AVAILABLE = 7,
            NOT_BOARDABLE = 8
        }

        /** Properties of a CarriageDetails. */
        interface ICarriageDetails {

            /** CarriageDetails id */
            id?: (string|null);

            /** CarriageDetails label */
            label?: (string|null);

            /** CarriageDetails occupancyStatus */
            occupancyStatus?: (transit_realtime.VehiclePosition.OccupancyStatus|null);

            /** CarriageDetails occupancyPercentage */
            occupancyPercentage?: (number|null);

            /** CarriageDetails carriageSequence */
            carriageSequence?: (number|null);
        }

        /** Represents a CarriageDetails. */
        class CarriageDetails implements ICarriageDetails {

            /**
             * Constructs a new CarriageDetails.
             * @param [properties] Properties to set
             */
            constructor(properties?: transit_realtime.VehiclePosition.ICarriageDetails);

            /** CarriageDetails id. */
            public id: string;

            /** CarriageDetails label. */
            public label: string;

            /** CarriageDetails occupancyStatus. */
            public occupancyStatus: transit_realtime.VehiclePosition.OccupancyStatus;

            /** CarriageDetails occupancyPercentage. */
            public occupancyPercentage: number;

            /** CarriageDetails carriageSequence. */
            public carriageSequence: number;

            /**
             * Creates a new CarriageDetails instance using the specified properties.
             * @param [properties] Properties to set
             * @returns CarriageDetails instance
             */
            public static create(properties?: transit_realtime.VehiclePosition.ICarriageDetails): transit_realtime.VehiclePosition.CarriageDetails;

            /**
             * Encodes the specified CarriageDetails message. Does not implicitly {@link transit_realtime.VehiclePosition.CarriageDetails.verify|verify} messages.
             * @param message CarriageDetails message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: transit_realtime.VehiclePosition.ICarriageDetails, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified CarriageDetails message, length delimited. Does not implicitly {@link transit_realtime.VehiclePosition.CarriageDetails.verify|verify} messages.
             * @param message CarriageDetails message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: transit_realtime.VehiclePosition.ICarriageDetails, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a CarriageDetails message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns CarriageDetails
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): transit_realtime.VehiclePosition.CarriageDetails;

            /**
             * Decodes a CarriageDetails message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns CarriageDetails
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): transit_realtime.VehiclePosition.CarriageDetails;

            /**
             * Verifies a CarriageDetails message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a CarriageDetails message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns CarriageDetails
             */
            public static fromObject(object: { [k: string]: any }): transit_realtime.VehiclePosition.CarriageDetails;

            /**
             * Creates a plain object from a CarriageDetails message. Also converts values to other types if specified.
             * @param message CarriageDetails
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: transit_realtime.VehiclePosition.CarriageDetails, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this CarriageDetails to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }
    }

    /** Properties of an Alert. */
    interface IAlert {

        /** Alert activePeriod */
        activePeriod?: (transit_realtime.ITimeRange[]|null);

        /** Alert informedEntity */
        informedEntity?: (transit_realtime.IEntitySelector[]|null);

        /** Alert cause */
        cause?: (transit_realtime.Alert.Cause|null);

        /** Alert effect */
        effect?: (transit_realtime.Alert.Effect|null);

        /** Alert url */
        url?: (transit_realtime.ITranslatedString|null);

        /** Alert headerText */
        headerText?: (transit_realtime.ITranslatedString|null);

        /** Alert descriptionText */
        descriptionText?: (transit_realtime.ITranslatedString|null);

        /** Alert ttsHeaderText */
        ttsHeaderText?: (transit_realtime.ITranslatedString|null);

        /** Alert ttsDescriptionText */
        ttsDescriptionText?: (transit_realtime.ITranslatedString|null);

        /** Alert severityLevel */
        severityLevel?: (transit_realtime.Alert.SeverityLevel|null);

        /** Alert image */
        image?: (transit_realtime.ITranslatedImage|null);

        /** Alert imageAlternativeText */
        imageAlternativeText?: (transit_realtime.ITranslatedString|null);

        /** Alert .transitAlertExtension */
        ".transitAlertExtension"?: (ITransitAlertExtension|null);
    }

    /** Represents an Alert. */
    class Alert implements IAlert {

        /**
         * Constructs a new Alert.
         * @param [properties] Properties to set
         */
        constructor(properties?: transit_realtime.IAlert);

        /** Alert activePeriod. */
        public activePeriod: transit_realtime.ITimeRange[];

        /** Alert informedEntity. */
        public informedEntity: transit_realtime.IEntitySelector[];

        /** Alert cause. */
        public cause: transit_realtime.Alert.Cause;

        /** Alert effect. */
        public effect: transit_realtime.Alert.Effect;

        /** Alert url. */
        public url?: (transit_realtime.ITranslatedString|null);

        /** Alert headerText. */
        public headerText?: (transit_realtime.ITranslatedString|null);

        /** Alert descriptionText. */
        public descriptionText?: (transit_realtime.ITranslatedString|null);

        /** Alert ttsHeaderText. */
        public ttsHeaderText?: (transit_realtime.ITranslatedString|null);

        /** Alert ttsDescriptionText. */
        public ttsDescriptionText?: (transit_realtime.ITranslatedString|null);

        /** Alert severityLevel. */
        public severityLevel: transit_realtime.Alert.SeverityLevel;

        /** Alert image. */
        public image?: (transit_realtime.ITranslatedImage|null);

        /** Alert imageAlternativeText. */
        public imageAlternativeText?: (transit_realtime.ITranslatedString|null);

        /** Alert .transitAlertExtension. */
        public transitAlertExtension?: (ITransitAlertExtension|null);

        /**
         * Creates a new Alert instance using the specified properties.
         * @param [properties] Properties to set
         * @returns Alert instance
         */
        public static create(properties?: transit_realtime.IAlert): transit_realtime.Alert;

        /**
         * Encodes the specified Alert message. Does not implicitly {@link transit_realtime.Alert.verify|verify} messages.
         * @param message Alert message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encode(message: transit_realtime.IAlert, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Encodes the specified Alert message, length delimited. Does not implicitly {@link transit_realtime.Alert.verify|verify} messages.
         * @param message Alert message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encodeDelimited(message: transit_realtime.IAlert, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Decodes an Alert message from the specified reader or buffer.
         * @param reader Reader or buffer to decode from
         * @param [length] Message length if known beforehand
         * @returns Alert
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): transit_realtime.Alert;

        /**
         * Decodes an Alert message from the specified reader or buffer, length delimited.
         * @param reader Reader or buffer to decode from
         * @returns Alert
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): transit_realtime.Alert;

        /**
         * Verifies an Alert message.
         * @param message Plain object to verify
         * @returns `null` if valid, otherwise the reason why it is not
         */
        public static verify(message: { [k: string]: any }): (string|null);

        /**
         * Creates an Alert message from a plain object. Also converts values to their respective internal types.
         * @param object Plain object
         * @returns Alert
         */
        public static fromObject(object: { [k: string]: any }): transit_realtime.Alert;

        /**
         * Creates a plain object from an Alert message. Also converts values to other types if specified.
         * @param message Alert
         * @param [options] Conversion options
         * @returns Plain object
         */
        public static toObject(message: transit_realtime.Alert, options?: $protobuf.IConversionOptions): { [k: string]: any };

        /**
         * Converts this Alert to JSON.
         * @returns JSON object
         */
        public toJSON(): { [k: string]: any };
    }

    namespace Alert {

        /** Cause enum. */
        enum Cause {
            UNKNOWN_CAUSE = 1,
            OTHER_CAUSE = 2,
            TECHNICAL_PROBLEM = 3,
            STRIKE = 4,
            DEMONSTRATION = 5,
            ACCIDENT = 6,
            HOLIDAY = 7,
            WEATHER = 8,
            MAINTENANCE = 9,
            CONSTRUCTION = 10,
            POLICE_ACTIVITY = 11,
            MEDICAL_EMERGENCY = 12
        }

        /** Effect enum. */
        enum Effect {
            NO_SERVICE = 1,
            REDUCED_SERVICE = 2,
            SIGNIFICANT_DELAYS = 3,
            DETOUR = 4,
            ADDITIONAL_SERVICE = 5,
            MODIFIED_SERVICE = 6,
            OTHER_EFFECT = 7,
            UNKNOWN_EFFECT = 8,
            STOP_MOVED = 9,
            NO_EFFECT = 10,
            ACCESSIBILITY_ISSUE = 11
        }

        /** SeverityLevel enum. */
        enum SeverityLevel {
            UNKNOWN_SEVERITY = 1,
            INFO = 2,
            WARNING = 3,
            SEVERE = 4
        }
    }

    /** Properties of a TimeRange. */
    interface ITimeRange {

        /** TimeRange start */
        start?: (number|Long|null);

        /** TimeRange end */
        end?: (number|Long|null);
    }

    /** Represents a TimeRange. */
    class TimeRange implements ITimeRange {

        /**
         * Constructs a new TimeRange.
         * @param [properties] Properties to set
         */
        constructor(properties?: transit_realtime.ITimeRange);

        /** TimeRange start. */
        public start: (number|Long);

        /** TimeRange end. */
        public end: (number|Long);

        /**
         * Creates a new TimeRange instance using the specified properties.
         * @param [properties] Properties to set
         * @returns TimeRange instance
         */
        public static create(properties?: transit_realtime.ITimeRange): transit_realtime.TimeRange;

        /**
         * Encodes the specified TimeRange message. Does not implicitly {@link transit_realtime.TimeRange.verify|verify} messages.
         * @param message TimeRange message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encode(message: transit_realtime.ITimeRange, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Encodes the specified TimeRange message, length delimited. Does not implicitly {@link transit_realtime.TimeRange.verify|verify} messages.
         * @param message TimeRange message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encodeDelimited(message: transit_realtime.ITimeRange, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Decodes a TimeRange message from the specified reader or buffer.
         * @param reader Reader or buffer to decode from
         * @param [length] Message length if known beforehand
         * @returns TimeRange
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): transit_realtime.TimeRange;

        /**
         * Decodes a TimeRange message from the specified reader or buffer, length delimited.
         * @param reader Reader or buffer to decode from
         * @returns TimeRange
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): transit_realtime.TimeRange;

        /**
         * Verifies a TimeRange message.
         * @param message Plain object to verify
         * @returns `null` if valid, otherwise the reason why it is not
         */
        public static verify(message: { [k: string]: any }): (string|null);

        /**
         * Creates a TimeRange message from a plain object. Also converts values to their respective internal types.
         * @param object Plain object
         * @returns TimeRange
         */
        public static fromObject(object: { [k: string]: any }): transit_realtime.TimeRange;

        /**
         * Creates a plain object from a TimeRange message. Also converts values to other types if specified.
         * @param message TimeRange
         * @param [options] Conversion options
         * @returns Plain object
         */
        public static toObject(message: transit_realtime.TimeRange, options?: $protobuf.IConversionOptions): { [k: string]: any };

        /**
         * Converts this TimeRange to JSON.
         * @returns JSON object
         */
        public toJSON(): { [k: string]: any };
    }

    /** Properties of a Position. */
    interface IPosition {

        /** Position latitude */
        latitude: number;

        /** Position longitude */
        longitude: number;

        /** Position bearing */
        bearing?: (number|null);

        /** Position odometer */
        odometer?: (number|null);

        /** Position speed */
        speed?: (number|null);
    }

    /** Represents a Position. */
    class Position implements IPosition {

        /**
         * Constructs a new Position.
         * @param [properties] Properties to set
         */
        constructor(properties?: transit_realtime.IPosition);

        /** Position latitude. */
        public latitude: number;

        /** Position longitude. */
        public longitude: number;

        /** Position bearing. */
        public bearing: number;

        /** Position odometer. */
        public odometer: number;

        /** Position speed. */
        public speed: number;

        /**
         * Creates a new Position instance using the specified properties.
         * @param [properties] Properties to set
         * @returns Position instance
         */
        public static create(properties?: transit_realtime.IPosition): transit_realtime.Position;

        /**
         * Encodes the specified Position message. Does not implicitly {@link transit_realtime.Position.verify|verify} messages.
         * @param message Position message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encode(message: transit_realtime.IPosition, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Encodes the specified Position message, length delimited. Does not implicitly {@link transit_realtime.Position.verify|verify} messages.
         * @param message Position message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encodeDelimited(message: transit_realtime.IPosition, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Decodes a Position message from the specified reader or buffer.
         * @param reader Reader or buffer to decode from
         * @param [length] Message length if known beforehand
         * @returns Position
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): transit_realtime.Position;

        /**
         * Decodes a Position message from the specified reader or buffer, length delimited.
         * @param reader Reader or buffer to decode from
         * @returns Position
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): transit_realtime.Position;

        /**
         * Verifies a Position message.
         * @param message Plain object to verify
         * @returns `null` if valid, otherwise the reason why it is not
         */
        public static verify(message: { [k: string]: any }): (string|null);

        /**
         * Creates a Position message from a plain object. Also converts values to their respective internal types.
         * @param object Plain object
         * @returns Position
         */
        public static fromObject(object: { [k: string]: any }): transit_realtime.Position;

        /**
         * Creates a plain object from a Position message. Also converts values to other types if specified.
         * @param message Position
         * @param [options] Conversion options
         * @returns Plain object
         */
        public static toObject(message: transit_realtime.Position, options?: $protobuf.IConversionOptions): { [k: string]: any };

        /**
         * Converts this Position to JSON.
         * @returns JSON object
         */
        public toJSON(): { [k: string]: any };
    }

    /** Properties of a TripDescriptor. */
    interface ITripDescriptor {

        /** TripDescriptor tripId */
        tripId?: (string|null);

        /** TripDescriptor routeId */
        routeId?: (string|null);

        /** TripDescriptor directionId */
        directionId?: (number|null);

        /** TripDescriptor startTime */
        startTime?: (string|null);

        /** TripDescriptor startDate */
        startDate?: (string|null);

        /** TripDescriptor scheduleRelationship */
        scheduleRelationship?: (transit_realtime.TripDescriptor.ScheduleRelationship|null);

        /** TripDescriptor .transitTripDescriptorExtension */
        ".transitTripDescriptorExtension"?: (ITransitTripDescriptorExtension|null);

        /** TripDescriptor .nyctTripDescriptor */
        ".nyctTripDescriptor"?: (INyctTripDescriptor|null);
    }

    /** Represents a TripDescriptor. */
    class TripDescriptor implements ITripDescriptor {

        /**
         * Constructs a new TripDescriptor.
         * @param [properties] Properties to set
         */
        constructor(properties?: transit_realtime.ITripDescriptor);

        /** TripDescriptor tripId. */
        public tripId: string;

        /** TripDescriptor routeId. */
        public routeId: string;

        /** TripDescriptor directionId. */
        public directionId: number;

        /** TripDescriptor startTime. */
        public startTime: string;

        /** TripDescriptor startDate. */
        public startDate: string;

        /** TripDescriptor scheduleRelationship. */
        public scheduleRelationship: transit_realtime.TripDescriptor.ScheduleRelationship;

        /** TripDescriptor .transitTripDescriptorExtension. */
        public transitTripDescriptorExtension?: (ITransitTripDescriptorExtension|null);

        /** TripDescriptor .nyctTripDescriptor. */
        public nyctTripDescriptor?: (INyctTripDescriptor|null);

        /**
         * Creates a new TripDescriptor instance using the specified properties.
         * @param [properties] Properties to set
         * @returns TripDescriptor instance
         */
        public static create(properties?: transit_realtime.ITripDescriptor): transit_realtime.TripDescriptor;

        /**
         * Encodes the specified TripDescriptor message. Does not implicitly {@link transit_realtime.TripDescriptor.verify|verify} messages.
         * @param message TripDescriptor message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encode(message: transit_realtime.ITripDescriptor, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Encodes the specified TripDescriptor message, length delimited. Does not implicitly {@link transit_realtime.TripDescriptor.verify|verify} messages.
         * @param message TripDescriptor message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encodeDelimited(message: transit_realtime.ITripDescriptor, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Decodes a TripDescriptor message from the specified reader or buffer.
         * @param reader Reader or buffer to decode from
         * @param [length] Message length if known beforehand
         * @returns TripDescriptor
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): transit_realtime.TripDescriptor;

        /**
         * Decodes a TripDescriptor message from the specified reader or buffer, length delimited.
         * @param reader Reader or buffer to decode from
         * @returns TripDescriptor
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): transit_realtime.TripDescriptor;

        /**
         * Verifies a TripDescriptor message.
         * @param message Plain object to verify
         * @returns `null` if valid, otherwise the reason why it is not
         */
        public static verify(message: { [k: string]: any }): (string|null);

        /**
         * Creates a TripDescriptor message from a plain object. Also converts values to their respective internal types.
         * @param object Plain object
         * @returns TripDescriptor
         */
        public static fromObject(object: { [k: string]: any }): transit_realtime.TripDescriptor;

        /**
         * Creates a plain object from a TripDescriptor message. Also converts values to other types if specified.
         * @param message TripDescriptor
         * @param [options] Conversion options
         * @returns Plain object
         */
        public static toObject(message: transit_realtime.TripDescriptor, options?: $protobuf.IConversionOptions): { [k: string]: any };

        /**
         * Converts this TripDescriptor to JSON.
         * @returns JSON object
         */
        public toJSON(): { [k: string]: any };
    }

    namespace TripDescriptor {

        /** ScheduleRelationship enum. */
        enum ScheduleRelationship {
            SCHEDULED = 0,
            ADDED = 1,
            UNSCHEDULED = 2,
            CANCELED = 3,
            REPLACEMENT = 5,
            DUPLICATED = 6
        }
    }

    /** Properties of a VehicleDescriptor. */
    interface IVehicleDescriptor {

        /** VehicleDescriptor id */
        id?: (string|null);

        /** VehicleDescriptor label */
        label?: (string|null);

        /** VehicleDescriptor licensePlate */
        licensePlate?: (string|null);

        /** VehicleDescriptor .transitVehicleDescriptorExtension */
        ".transitVehicleDescriptorExtension"?: (ITransitVehicleDescriptorExtension|null);

        /** VehicleDescriptor .tfnswVehicleDescriptor */
        ".tfnswVehicleDescriptor"?: (ITfnswVehicleDescriptor|null);
    }

    /** Represents a VehicleDescriptor. */
    class VehicleDescriptor implements IVehicleDescriptor {

        /**
         * Constructs a new VehicleDescriptor.
         * @param [properties] Properties to set
         */
        constructor(properties?: transit_realtime.IVehicleDescriptor);

        /** VehicleDescriptor id. */
        public id: string;

        /** VehicleDescriptor label. */
        public label: string;

        /** VehicleDescriptor licensePlate. */
        public licensePlate: string;

        /** VehicleDescriptor .transitVehicleDescriptorExtension. */
        public transitVehicleDescriptorExtension?: (ITransitVehicleDescriptorExtension|null);

        /** VehicleDescriptor .tfnswVehicleDescriptor. */
        public tfnswVehicleDescriptor?: (ITfnswVehicleDescriptor|null);

        /**
         * Creates a new VehicleDescriptor instance using the specified properties.
         * @param [properties] Properties to set
         * @returns VehicleDescriptor instance
         */
        public static create(properties?: transit_realtime.IVehicleDescriptor): transit_realtime.VehicleDescriptor;

        /**
         * Encodes the specified VehicleDescriptor message. Does not implicitly {@link transit_realtime.VehicleDescriptor.verify|verify} messages.
         * @param message VehicleDescriptor message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encode(message: transit_realtime.IVehicleDescriptor, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Encodes the specified VehicleDescriptor message, length delimited. Does not implicitly {@link transit_realtime.VehicleDescriptor.verify|verify} messages.
         * @param message VehicleDescriptor message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encodeDelimited(message: transit_realtime.IVehicleDescriptor, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Decodes a VehicleDescriptor message from the specified reader or buffer.
         * @param reader Reader or buffer to decode from
         * @param [length] Message length if known beforehand
         * @returns VehicleDescriptor
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): transit_realtime.VehicleDescriptor;

        /**
         * Decodes a VehicleDescriptor message from the specified reader or buffer, length delimited.
         * @param reader Reader or buffer to decode from
         * @returns VehicleDescriptor
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): transit_realtime.VehicleDescriptor;

        /**
         * Verifies a VehicleDescriptor message.
         * @param message Plain object to verify
         * @returns `null` if valid, otherwise the reason why it is not
         */
        public static verify(message: { [k: string]: any }): (string|null);

        /**
         * Creates a VehicleDescriptor message from a plain object. Also converts values to their respective internal types.
         * @param object Plain object
         * @returns VehicleDescriptor
         */
        public static fromObject(object: { [k: string]: any }): transit_realtime.VehicleDescriptor;

        /**
         * Creates a plain object from a VehicleDescriptor message. Also converts values to other types if specified.
         * @param message VehicleDescriptor
         * @param [options] Conversion options
         * @returns Plain object
         */
        public static toObject(message: transit_realtime.VehicleDescriptor, options?: $protobuf.IConversionOptions): { [k: string]: any };

        /**
         * Converts this VehicleDescriptor to JSON.
         * @returns JSON object
         */
        public toJSON(): { [k: string]: any };
    }

    /** Properties of an EntitySelector. */
    interface IEntitySelector {

        /** EntitySelector agencyId */
        agencyId?: (string|null);

        /** EntitySelector routeId */
        routeId?: (string|null);

        /** EntitySelector routeType */
        routeType?: (number|null);

        /** EntitySelector trip */
        trip?: (transit_realtime.ITripDescriptor|null);

        /** EntitySelector stopId */
        stopId?: (string|null);

        /** EntitySelector directionId */
        directionId?: (number|null);

        /** EntitySelector .transitEntitySelectorExtension */
        ".transitEntitySelectorExtension"?: (ITransitInformedEntityExtension|null);
    }

    /** Represents an EntitySelector. */
    class EntitySelector implements IEntitySelector {

        /**
         * Constructs a new EntitySelector.
         * @param [properties] Properties to set
         */
        constructor(properties?: transit_realtime.IEntitySelector);

        /** EntitySelector agencyId. */
        public agencyId: string;

        /** EntitySelector routeId. */
        public routeId: string;

        /** EntitySelector routeType. */
        public routeType: number;

        /** EntitySelector trip. */
        public trip?: (transit_realtime.ITripDescriptor|null);

        /** EntitySelector stopId. */
        public stopId: string;

        /** EntitySelector directionId. */
        public directionId: number;

        /** EntitySelector .transitEntitySelectorExtension. */
        public transitEntitySelectorExtension?: (ITransitInformedEntityExtension|null);

        /**
         * Creates a new EntitySelector instance using the specified properties.
         * @param [properties] Properties to set
         * @returns EntitySelector instance
         */
        public static create(properties?: transit_realtime.IEntitySelector): transit_realtime.EntitySelector;

        /**
         * Encodes the specified EntitySelector message. Does not implicitly {@link transit_realtime.EntitySelector.verify|verify} messages.
         * @param message EntitySelector message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encode(message: transit_realtime.IEntitySelector, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Encodes the specified EntitySelector message, length delimited. Does not implicitly {@link transit_realtime.EntitySelector.verify|verify} messages.
         * @param message EntitySelector message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encodeDelimited(message: transit_realtime.IEntitySelector, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Decodes an EntitySelector message from the specified reader or buffer.
         * @param reader Reader or buffer to decode from
         * @param [length] Message length if known beforehand
         * @returns EntitySelector
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): transit_realtime.EntitySelector;

        /**
         * Decodes an EntitySelector message from the specified reader or buffer, length delimited.
         * @param reader Reader or buffer to decode from
         * @returns EntitySelector
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): transit_realtime.EntitySelector;

        /**
         * Verifies an EntitySelector message.
         * @param message Plain object to verify
         * @returns `null` if valid, otherwise the reason why it is not
         */
        public static verify(message: { [k: string]: any }): (string|null);

        /**
         * Creates an EntitySelector message from a plain object. Also converts values to their respective internal types.
         * @param object Plain object
         * @returns EntitySelector
         */
        public static fromObject(object: { [k: string]: any }): transit_realtime.EntitySelector;

        /**
         * Creates a plain object from an EntitySelector message. Also converts values to other types if specified.
         * @param message EntitySelector
         * @param [options] Conversion options
         * @returns Plain object
         */
        public static toObject(message: transit_realtime.EntitySelector, options?: $protobuf.IConversionOptions): { [k: string]: any };

        /**
         * Converts this EntitySelector to JSON.
         * @returns JSON object
         */
        public toJSON(): { [k: string]: any };
    }

    /** Properties of a TranslatedString. */
    interface ITranslatedString {

        /** TranslatedString translation */
        translation?: (transit_realtime.TranslatedString.ITranslation[]|null);
    }

    /** Represents a TranslatedString. */
    class TranslatedString implements ITranslatedString {

        /**
         * Constructs a new TranslatedString.
         * @param [properties] Properties to set
         */
        constructor(properties?: transit_realtime.ITranslatedString);

        /** TranslatedString translation. */
        public translation: transit_realtime.TranslatedString.ITranslation[];

        /**
         * Creates a new TranslatedString instance using the specified properties.
         * @param [properties] Properties to set
         * @returns TranslatedString instance
         */
        public static create(properties?: transit_realtime.ITranslatedString): transit_realtime.TranslatedString;

        /**
         * Encodes the specified TranslatedString message. Does not implicitly {@link transit_realtime.TranslatedString.verify|verify} messages.
         * @param message TranslatedString message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encode(message: transit_realtime.ITranslatedString, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Encodes the specified TranslatedString message, length delimited. Does not implicitly {@link transit_realtime.TranslatedString.verify|verify} messages.
         * @param message TranslatedString message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encodeDelimited(message: transit_realtime.ITranslatedString, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Decodes a TranslatedString message from the specified reader or buffer.
         * @param reader Reader or buffer to decode from
         * @param [length] Message length if known beforehand
         * @returns TranslatedString
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): transit_realtime.TranslatedString;

        /**
         * Decodes a TranslatedString message from the specified reader or buffer, length delimited.
         * @param reader Reader or buffer to decode from
         * @returns TranslatedString
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): transit_realtime.TranslatedString;

        /**
         * Verifies a TranslatedString message.
         * @param message Plain object to verify
         * @returns `null` if valid, otherwise the reason why it is not
         */
        public static verify(message: { [k: string]: any }): (string|null);

        /**
         * Creates a TranslatedString message from a plain object. Also converts values to their respective internal types.
         * @param object Plain object
         * @returns TranslatedString
         */
        public static fromObject(object: { [k: string]: any }): transit_realtime.TranslatedString;

        /**
         * Creates a plain object from a TranslatedString message. Also converts values to other types if specified.
         * @param message TranslatedString
         * @param [options] Conversion options
         * @returns Plain object
         */
        public static toObject(message: transit_realtime.TranslatedString, options?: $protobuf.IConversionOptions): { [k: string]: any };

        /**
         * Converts this TranslatedString to JSON.
         * @returns JSON object
         */
        public toJSON(): { [k: string]: any };
    }

    namespace TranslatedString {

        /** Properties of a Translation. */
        interface ITranslation {

            /** Translation text */
            text: string;

            /** Translation language */
            language?: (string|null);
        }

        /** Represents a Translation. */
        class Translation implements ITranslation {

            /**
             * Constructs a new Translation.
             * @param [properties] Properties to set
             */
            constructor(properties?: transit_realtime.TranslatedString.ITranslation);

            /** Translation text. */
            public text: string;

            /** Translation language. */
            public language: string;

            /**
             * Creates a new Translation instance using the specified properties.
             * @param [properties] Properties to set
             * @returns Translation instance
             */
            public static create(properties?: transit_realtime.TranslatedString.ITranslation): transit_realtime.TranslatedString.Translation;

            /**
             * Encodes the specified Translation message. Does not implicitly {@link transit_realtime.TranslatedString.Translation.verify|verify} messages.
             * @param message Translation message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: transit_realtime.TranslatedString.ITranslation, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified Translation message, length delimited. Does not implicitly {@link transit_realtime.TranslatedString.Translation.verify|verify} messages.
             * @param message Translation message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: transit_realtime.TranslatedString.ITranslation, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a Translation message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns Translation
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): transit_realtime.TranslatedString.Translation;

            /**
             * Decodes a Translation message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns Translation
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): transit_realtime.TranslatedString.Translation;

            /**
             * Verifies a Translation message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a Translation message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns Translation
             */
            public static fromObject(object: { [k: string]: any }): transit_realtime.TranslatedString.Translation;

            /**
             * Creates a plain object from a Translation message. Also converts values to other types if specified.
             * @param message Translation
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: transit_realtime.TranslatedString.Translation, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this Translation to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }
    }

    /** Properties of a TranslatedImage. */
    interface ITranslatedImage {

        /** TranslatedImage localizedImage */
        localizedImage?: (transit_realtime.TranslatedImage.ILocalizedImage[]|null);
    }

    /** Represents a TranslatedImage. */
    class TranslatedImage implements ITranslatedImage {

        /**
         * Constructs a new TranslatedImage.
         * @param [properties] Properties to set
         */
        constructor(properties?: transit_realtime.ITranslatedImage);

        /** TranslatedImage localizedImage. */
        public localizedImage: transit_realtime.TranslatedImage.ILocalizedImage[];

        /**
         * Creates a new TranslatedImage instance using the specified properties.
         * @param [properties] Properties to set
         * @returns TranslatedImage instance
         */
        public static create(properties?: transit_realtime.ITranslatedImage): transit_realtime.TranslatedImage;

        /**
         * Encodes the specified TranslatedImage message. Does not implicitly {@link transit_realtime.TranslatedImage.verify|verify} messages.
         * @param message TranslatedImage message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encode(message: transit_realtime.ITranslatedImage, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Encodes the specified TranslatedImage message, length delimited. Does not implicitly {@link transit_realtime.TranslatedImage.verify|verify} messages.
         * @param message TranslatedImage message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encodeDelimited(message: transit_realtime.ITranslatedImage, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Decodes a TranslatedImage message from the specified reader or buffer.
         * @param reader Reader or buffer to decode from
         * @param [length] Message length if known beforehand
         * @returns TranslatedImage
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): transit_realtime.TranslatedImage;

        /**
         * Decodes a TranslatedImage message from the specified reader or buffer, length delimited.
         * @param reader Reader or buffer to decode from
         * @returns TranslatedImage
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): transit_realtime.TranslatedImage;

        /**
         * Verifies a TranslatedImage message.
         * @param message Plain object to verify
         * @returns `null` if valid, otherwise the reason why it is not
         */
        public static verify(message: { [k: string]: any }): (string|null);

        /**
         * Creates a TranslatedImage message from a plain object. Also converts values to their respective internal types.
         * @param object Plain object
         * @returns TranslatedImage
         */
        public static fromObject(object: { [k: string]: any }): transit_realtime.TranslatedImage;

        /**
         * Creates a plain object from a TranslatedImage message. Also converts values to other types if specified.
         * @param message TranslatedImage
         * @param [options] Conversion options
         * @returns Plain object
         */
        public static toObject(message: transit_realtime.TranslatedImage, options?: $protobuf.IConversionOptions): { [k: string]: any };

        /**
         * Converts this TranslatedImage to JSON.
         * @returns JSON object
         */
        public toJSON(): { [k: string]: any };
    }

    namespace TranslatedImage {

        /** Properties of a LocalizedImage. */
        interface ILocalizedImage {

            /** LocalizedImage url */
            url: string;

            /** LocalizedImage mediaType */
            mediaType: string;

            /** LocalizedImage language */
            language?: (string|null);
        }

        /** Represents a LocalizedImage. */
        class LocalizedImage implements ILocalizedImage {

            /**
             * Constructs a new LocalizedImage.
             * @param [properties] Properties to set
             */
            constructor(properties?: transit_realtime.TranslatedImage.ILocalizedImage);

            /** LocalizedImage url. */
            public url: string;

            /** LocalizedImage mediaType. */
            public mediaType: string;

            /** LocalizedImage language. */
            public language: string;

            /**
             * Creates a new LocalizedImage instance using the specified properties.
             * @param [properties] Properties to set
             * @returns LocalizedImage instance
             */
            public static create(properties?: transit_realtime.TranslatedImage.ILocalizedImage): transit_realtime.TranslatedImage.LocalizedImage;

            /**
             * Encodes the specified LocalizedImage message. Does not implicitly {@link transit_realtime.TranslatedImage.LocalizedImage.verify|verify} messages.
             * @param message LocalizedImage message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: transit_realtime.TranslatedImage.ILocalizedImage, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified LocalizedImage message, length delimited. Does not implicitly {@link transit_realtime.TranslatedImage.LocalizedImage.verify|verify} messages.
             * @param message LocalizedImage message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: transit_realtime.TranslatedImage.ILocalizedImage, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a LocalizedImage message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns LocalizedImage
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): transit_realtime.TranslatedImage.LocalizedImage;

            /**
             * Decodes a LocalizedImage message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns LocalizedImage
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): transit_realtime.TranslatedImage.LocalizedImage;

            /**
             * Verifies a LocalizedImage message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a LocalizedImage message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns LocalizedImage
             */
            public static fromObject(object: { [k: string]: any }): transit_realtime.TranslatedImage.LocalizedImage;

            /**
             * Creates a plain object from a LocalizedImage message. Also converts values to other types if specified.
             * @param message LocalizedImage
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: transit_realtime.TranslatedImage.LocalizedImage, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this LocalizedImage to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }
    }

    /** Properties of a Shape. */
    interface IShape {

        /** Shape shapeId */
        shapeId?: (string|null);

        /** Shape encodedPolyline */
        encodedPolyline?: (string|null);
    }

    /** Represents a Shape. */
    class Shape implements IShape {

        /**
         * Constructs a new Shape.
         * @param [properties] Properties to set
         */
        constructor(properties?: transit_realtime.IShape);

        /** Shape shapeId. */
        public shapeId: string;

        /** Shape encodedPolyline. */
        public encodedPolyline: string;

        /**
         * Creates a new Shape instance using the specified properties.
         * @param [properties] Properties to set
         * @returns Shape instance
         */
        public static create(properties?: transit_realtime.IShape): transit_realtime.Shape;

        /**
         * Encodes the specified Shape message. Does not implicitly {@link transit_realtime.Shape.verify|verify} messages.
         * @param message Shape message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encode(message: transit_realtime.IShape, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Encodes the specified Shape message, length delimited. Does not implicitly {@link transit_realtime.Shape.verify|verify} messages.
         * @param message Shape message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encodeDelimited(message: transit_realtime.IShape, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Decodes a Shape message from the specified reader or buffer.
         * @param reader Reader or buffer to decode from
         * @param [length] Message length if known beforehand
         * @returns Shape
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): transit_realtime.Shape;

        /**
         * Decodes a Shape message from the specified reader or buffer, length delimited.
         * @param reader Reader or buffer to decode from
         * @returns Shape
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): transit_realtime.Shape;

        /**
         * Verifies a Shape message.
         * @param message Plain object to verify
         * @returns `null` if valid, otherwise the reason why it is not
         */
        public static verify(message: { [k: string]: any }): (string|null);

        /**
         * Creates a Shape message from a plain object. Also converts values to their respective internal types.
         * @param object Plain object
         * @returns Shape
         */
        public static fromObject(object: { [k: string]: any }): transit_realtime.Shape;

        /**
         * Creates a plain object from a Shape message. Also converts values to other types if specified.
         * @param message Shape
         * @param [options] Conversion options
         * @returns Plain object
         */
        public static toObject(message: transit_realtime.Shape, options?: $protobuf.IConversionOptions): { [k: string]: any };

        /**
         * Converts this Shape to JSON.
         * @returns JSON object
         */
        public toJSON(): { [k: string]: any };
    }
}

/** Properties of a TfnswVehicleDescriptor. */
export interface ITfnswVehicleDescriptor {

    /** TfnswVehicleDescriptor airConditioned */
    airConditioned?: (boolean|null);

    /** TfnswVehicleDescriptor wheelchairAccessible */
    wheelchairAccessible?: (number|null);
}

/** Represents a TfnswVehicleDescriptor. */
export class TfnswVehicleDescriptor implements ITfnswVehicleDescriptor {

    /**
     * Constructs a new TfnswVehicleDescriptor.
     * @param [properties] Properties to set
     */
    constructor(properties?: ITfnswVehicleDescriptor);

    /** TfnswVehicleDescriptor airConditioned. */
    public airConditioned: boolean;

    /** TfnswVehicleDescriptor wheelchairAccessible. */
    public wheelchairAccessible: number;

    /**
     * Creates a new TfnswVehicleDescriptor instance using the specified properties.
     * @param [properties] Properties to set
     * @returns TfnswVehicleDescriptor instance
     */
    public static create(properties?: ITfnswVehicleDescriptor): TfnswVehicleDescriptor;

    /**
     * Encodes the specified TfnswVehicleDescriptor message. Does not implicitly {@link TfnswVehicleDescriptor.verify|verify} messages.
     * @param message TfnswVehicleDescriptor message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encode(message: ITfnswVehicleDescriptor, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Encodes the specified TfnswVehicleDescriptor message, length delimited. Does not implicitly {@link TfnswVehicleDescriptor.verify|verify} messages.
     * @param message TfnswVehicleDescriptor message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encodeDelimited(message: ITfnswVehicleDescriptor, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Decodes a TfnswVehicleDescriptor message from the specified reader or buffer.
     * @param reader Reader or buffer to decode from
     * @param [length] Message length if known beforehand
     * @returns TfnswVehicleDescriptor
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): TfnswVehicleDescriptor;

    /**
     * Decodes a TfnswVehicleDescriptor message from the specified reader or buffer, length delimited.
     * @param reader Reader or buffer to decode from
     * @returns TfnswVehicleDescriptor
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): TfnswVehicleDescriptor;

    /**
     * Verifies a TfnswVehicleDescriptor message.
     * @param message Plain object to verify
     * @returns `null` if valid, otherwise the reason why it is not
     */
    public static verify(message: { [k: string]: any }): (string|null);

    /**
     * Creates a TfnswVehicleDescriptor message from a plain object. Also converts values to their respective internal types.
     * @param object Plain object
     * @returns TfnswVehicleDescriptor
     */
    public static fromObject(object: { [k: string]: any }): TfnswVehicleDescriptor;

    /**
     * Creates a plain object from a TfnswVehicleDescriptor message. Also converts values to other types if specified.
     * @param message TfnswVehicleDescriptor
     * @param [options] Conversion options
     * @returns Plain object
     */
    public static toObject(message: TfnswVehicleDescriptor, options?: $protobuf.IConversionOptions): { [k: string]: any };

    /**
     * Converts this TfnswVehicleDescriptor to JSON.
     * @returns JSON object
     */
    public toJSON(): { [k: string]: any };
}

/** Properties of a TripReplacementPeriod. */
export interface ITripReplacementPeriod {

    /** TripReplacementPeriod routeId */
    routeId?: (string|null);

    /** TripReplacementPeriod replacementPeriod */
    replacementPeriod?: (transit_realtime.ITimeRange|null);
}

/** Represents a TripReplacementPeriod. */
export class TripReplacementPeriod implements ITripReplacementPeriod {

    /**
     * Constructs a new TripReplacementPeriod.
     * @param [properties] Properties to set
     */
    constructor(properties?: ITripReplacementPeriod);

    /** TripReplacementPeriod routeId. */
    public routeId: string;

    /** TripReplacementPeriod replacementPeriod. */
    public replacementPeriod?: (transit_realtime.ITimeRange|null);

    /**
     * Creates a new TripReplacementPeriod instance using the specified properties.
     * @param [properties] Properties to set
     * @returns TripReplacementPeriod instance
     */
    public static create(properties?: ITripReplacementPeriod): TripReplacementPeriod;

    /**
     * Encodes the specified TripReplacementPeriod message. Does not implicitly {@link TripReplacementPeriod.verify|verify} messages.
     * @param message TripReplacementPeriod message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encode(message: ITripReplacementPeriod, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Encodes the specified TripReplacementPeriod message, length delimited. Does not implicitly {@link TripReplacementPeriod.verify|verify} messages.
     * @param message TripReplacementPeriod message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encodeDelimited(message: ITripReplacementPeriod, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Decodes a TripReplacementPeriod message from the specified reader or buffer.
     * @param reader Reader or buffer to decode from
     * @param [length] Message length if known beforehand
     * @returns TripReplacementPeriod
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): TripReplacementPeriod;

    /**
     * Decodes a TripReplacementPeriod message from the specified reader or buffer, length delimited.
     * @param reader Reader or buffer to decode from
     * @returns TripReplacementPeriod
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): TripReplacementPeriod;

    /**
     * Verifies a TripReplacementPeriod message.
     * @param message Plain object to verify
     * @returns `null` if valid, otherwise the reason why it is not
     */
    public static verify(message: { [k: string]: any }): (string|null);

    /**
     * Creates a TripReplacementPeriod message from a plain object. Also converts values to their respective internal types.
     * @param object Plain object
     * @returns TripReplacementPeriod
     */
    public static fromObject(object: { [k: string]: any }): TripReplacementPeriod;

    /**
     * Creates a plain object from a TripReplacementPeriod message. Also converts values to other types if specified.
     * @param message TripReplacementPeriod
     * @param [options] Conversion options
     * @returns Plain object
     */
    public static toObject(message: TripReplacementPeriod, options?: $protobuf.IConversionOptions): { [k: string]: any };

    /**
     * Converts this TripReplacementPeriod to JSON.
     * @returns JSON object
     */
    public toJSON(): { [k: string]: any };
}

/** Properties of a NyctFeedHeader. */
export interface INyctFeedHeader {

    /** NyctFeedHeader nyctSubwayVersion */
    nyctSubwayVersion: string;

    /** NyctFeedHeader tripReplacementPeriod */
    tripReplacementPeriod?: (ITripReplacementPeriod[]|null);
}

/** Represents a NyctFeedHeader. */
export class NyctFeedHeader implements INyctFeedHeader {

    /**
     * Constructs a new NyctFeedHeader.
     * @param [properties] Properties to set
     */
    constructor(properties?: INyctFeedHeader);

    /** NyctFeedHeader nyctSubwayVersion. */
    public nyctSubwayVersion: string;

    /** NyctFeedHeader tripReplacementPeriod. */
    public tripReplacementPeriod: ITripReplacementPeriod[];

    /**
     * Creates a new NyctFeedHeader instance using the specified properties.
     * @param [properties] Properties to set
     * @returns NyctFeedHeader instance
     */
    public static create(properties?: INyctFeedHeader): NyctFeedHeader;

    /**
     * Encodes the specified NyctFeedHeader message. Does not implicitly {@link NyctFeedHeader.verify|verify} messages.
     * @param message NyctFeedHeader message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encode(message: INyctFeedHeader, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Encodes the specified NyctFeedHeader message, length delimited. Does not implicitly {@link NyctFeedHeader.verify|verify} messages.
     * @param message NyctFeedHeader message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encodeDelimited(message: INyctFeedHeader, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Decodes a NyctFeedHeader message from the specified reader or buffer.
     * @param reader Reader or buffer to decode from
     * @param [length] Message length if known beforehand
     * @returns NyctFeedHeader
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): NyctFeedHeader;

    /**
     * Decodes a NyctFeedHeader message from the specified reader or buffer, length delimited.
     * @param reader Reader or buffer to decode from
     * @returns NyctFeedHeader
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): NyctFeedHeader;

    /**
     * Verifies a NyctFeedHeader message.
     * @param message Plain object to verify
     * @returns `null` if valid, otherwise the reason why it is not
     */
    public static verify(message: { [k: string]: any }): (string|null);

    /**
     * Creates a NyctFeedHeader message from a plain object. Also converts values to their respective internal types.
     * @param object Plain object
     * @returns NyctFeedHeader
     */
    public static fromObject(object: { [k: string]: any }): NyctFeedHeader;

    /**
     * Creates a plain object from a NyctFeedHeader message. Also converts values to other types if specified.
     * @param message NyctFeedHeader
     * @param [options] Conversion options
     * @returns Plain object
     */
    public static toObject(message: NyctFeedHeader, options?: $protobuf.IConversionOptions): { [k: string]: any };

    /**
     * Converts this NyctFeedHeader to JSON.
     * @returns JSON object
     */
    public toJSON(): { [k: string]: any };
}

/** Properties of a NyctTripDescriptor. */
export interface INyctTripDescriptor {

    /** NyctTripDescriptor trainId */
    trainId?: (string|null);

    /** NyctTripDescriptor isAssigned */
    isAssigned?: (boolean|null);

    /** NyctTripDescriptor direction */
    direction?: (NyctTripDescriptor.Direction|null);
}

/** Represents a NyctTripDescriptor. */
export class NyctTripDescriptor implements INyctTripDescriptor {

    /**
     * Constructs a new NyctTripDescriptor.
     * @param [properties] Properties to set
     */
    constructor(properties?: INyctTripDescriptor);

    /** NyctTripDescriptor trainId. */
    public trainId: string;

    /** NyctTripDescriptor isAssigned. */
    public isAssigned: boolean;

    /** NyctTripDescriptor direction. */
    public direction: NyctTripDescriptor.Direction;

    /**
     * Creates a new NyctTripDescriptor instance using the specified properties.
     * @param [properties] Properties to set
     * @returns NyctTripDescriptor instance
     */
    public static create(properties?: INyctTripDescriptor): NyctTripDescriptor;

    /**
     * Encodes the specified NyctTripDescriptor message. Does not implicitly {@link NyctTripDescriptor.verify|verify} messages.
     * @param message NyctTripDescriptor message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encode(message: INyctTripDescriptor, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Encodes the specified NyctTripDescriptor message, length delimited. Does not implicitly {@link NyctTripDescriptor.verify|verify} messages.
     * @param message NyctTripDescriptor message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encodeDelimited(message: INyctTripDescriptor, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Decodes a NyctTripDescriptor message from the specified reader or buffer.
     * @param reader Reader or buffer to decode from
     * @param [length] Message length if known beforehand
     * @returns NyctTripDescriptor
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): NyctTripDescriptor;

    /**
     * Decodes a NyctTripDescriptor message from the specified reader or buffer, length delimited.
     * @param reader Reader or buffer to decode from
     * @returns NyctTripDescriptor
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): NyctTripDescriptor;

    /**
     * Verifies a NyctTripDescriptor message.
     * @param message Plain object to verify
     * @returns `null` if valid, otherwise the reason why it is not
     */
    public static verify(message: { [k: string]: any }): (string|null);

    /**
     * Creates a NyctTripDescriptor message from a plain object. Also converts values to their respective internal types.
     * @param object Plain object
     * @returns NyctTripDescriptor
     */
    public static fromObject(object: { [k: string]: any }): NyctTripDescriptor;

    /**
     * Creates a plain object from a NyctTripDescriptor message. Also converts values to other types if specified.
     * @param message NyctTripDescriptor
     * @param [options] Conversion options
     * @returns Plain object
     */
    public static toObject(message: NyctTripDescriptor, options?: $protobuf.IConversionOptions): { [k: string]: any };

    /**
     * Converts this NyctTripDescriptor to JSON.
     * @returns JSON object
     */
    public toJSON(): { [k: string]: any };
}

export namespace NyctTripDescriptor {

    /** Direction enum. */
    enum Direction {
        NORTH = 1,
        EAST = 2,
        SOUTH = 3,
        WEST = 4
    }
}

/** Properties of a NyctStopTimeUpdate. */
export interface INyctStopTimeUpdate {

    /** NyctStopTimeUpdate scheduledTrack */
    scheduledTrack?: (string|null);

    /** NyctStopTimeUpdate actualTrack */
    actualTrack?: (string|null);
}

/** Represents a NyctStopTimeUpdate. */
export class NyctStopTimeUpdate implements INyctStopTimeUpdate {

    /**
     * Constructs a new NyctStopTimeUpdate.
     * @param [properties] Properties to set
     */
    constructor(properties?: INyctStopTimeUpdate);

    /** NyctStopTimeUpdate scheduledTrack. */
    public scheduledTrack: string;

    /** NyctStopTimeUpdate actualTrack. */
    public actualTrack: string;

    /**
     * Creates a new NyctStopTimeUpdate instance using the specified properties.
     * @param [properties] Properties to set
     * @returns NyctStopTimeUpdate instance
     */
    public static create(properties?: INyctStopTimeUpdate): NyctStopTimeUpdate;

    /**
     * Encodes the specified NyctStopTimeUpdate message. Does not implicitly {@link NyctStopTimeUpdate.verify|verify} messages.
     * @param message NyctStopTimeUpdate message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encode(message: INyctStopTimeUpdate, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Encodes the specified NyctStopTimeUpdate message, length delimited. Does not implicitly {@link NyctStopTimeUpdate.verify|verify} messages.
     * @param message NyctStopTimeUpdate message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encodeDelimited(message: INyctStopTimeUpdate, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Decodes a NyctStopTimeUpdate message from the specified reader or buffer.
     * @param reader Reader or buffer to decode from
     * @param [length] Message length if known beforehand
     * @returns NyctStopTimeUpdate
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): NyctStopTimeUpdate;

    /**
     * Decodes a NyctStopTimeUpdate message from the specified reader or buffer, length delimited.
     * @param reader Reader or buffer to decode from
     * @returns NyctStopTimeUpdate
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): NyctStopTimeUpdate;

    /**
     * Verifies a NyctStopTimeUpdate message.
     * @param message Plain object to verify
     * @returns `null` if valid, otherwise the reason why it is not
     */
    public static verify(message: { [k: string]: any }): (string|null);

    /**
     * Creates a NyctStopTimeUpdate message from a plain object. Also converts values to their respective internal types.
     * @param object Plain object
     * @returns NyctStopTimeUpdate
     */
    public static fromObject(object: { [k: string]: any }): NyctStopTimeUpdate;

    /**
     * Creates a plain object from a NyctStopTimeUpdate message. Also converts values to other types if specified.
     * @param message NyctStopTimeUpdate
     * @param [options] Conversion options
     * @returns Plain object
     */
    public static toObject(message: NyctStopTimeUpdate, options?: $protobuf.IConversionOptions): { [k: string]: any };

    /**
     * Converts this NyctStopTimeUpdate to JSON.
     * @returns JSON object
     */
    public toJSON(): { [k: string]: any };
}