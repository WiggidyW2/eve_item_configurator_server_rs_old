// @generated
/// Generated server implementations.
pub mod item_configurator_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ItemConfiguratorServer.
    #[async_trait]
    pub trait ItemConfigurator: Send + Sync + 'static {
        ///
        async fn update(
            &self,
            request: tonic::Request<super::UpdateReq>,
        ) -> Result<tonic::Response<super::UpdateRep>, tonic::Status>;
        ///
        async fn list(
            &self,
            request: tonic::Request<super::ListReq>,
        ) -> Result<tonic::Response<super::ListRep>, tonic::Status>;
        ///
        async fn list_characters(
            &self,
            request: tonic::Request<super::ListCharactersReq>,
        ) -> Result<tonic::Response<super::ListCharactersRep>, tonic::Status>;
        ///
        async fn add_characters(
            &self,
            request: tonic::Request<super::AddCharactersReq>,
        ) -> Result<tonic::Response<super::AddCharactersRep>, tonic::Status>;
        ///
        async fn del_characters(
            &self,
            request: tonic::Request<super::DelCharactersReq>,
        ) -> Result<tonic::Response<super::DelCharactersRep>, tonic::Status>;
    }
    ///
    #[derive(Debug)]
    pub struct ItemConfiguratorServer<T: ItemConfigurator> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ItemConfigurator> ItemConfiguratorServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ItemConfiguratorServer<T>
    where
        T: ItemConfigurator,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/item_configurator_proto.ItemConfigurator/Update" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateSvc<T: ItemConfigurator>(pub Arc<T>);
                    impl<
                        T: ItemConfigurator,
                    > tonic::server::UnaryService<super::UpdateReq> for UpdateSvc<T> {
                        type Response = super::UpdateRep;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/item_configurator_proto.ItemConfigurator/List" => {
                    #[allow(non_camel_case_types)]
                    struct ListSvc<T: ItemConfigurator>(pub Arc<T>);
                    impl<T: ItemConfigurator> tonic::server::UnaryService<super::ListReq>
                    for ListSvc<T> {
                        type Response = super::ListRep;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/item_configurator_proto.ItemConfigurator/ListCharacters" => {
                    #[allow(non_camel_case_types)]
                    struct ListCharactersSvc<T: ItemConfigurator>(pub Arc<T>);
                    impl<
                        T: ItemConfigurator,
                    > tonic::server::UnaryService<super::ListCharactersReq>
                    for ListCharactersSvc<T> {
                        type Response = super::ListCharactersRep;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListCharactersReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_characters(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListCharactersSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/item_configurator_proto.ItemConfigurator/AddCharacters" => {
                    #[allow(non_camel_case_types)]
                    struct AddCharactersSvc<T: ItemConfigurator>(pub Arc<T>);
                    impl<
                        T: ItemConfigurator,
                    > tonic::server::UnaryService<super::AddCharactersReq>
                    for AddCharactersSvc<T> {
                        type Response = super::AddCharactersRep;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddCharactersReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).add_characters(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddCharactersSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/item_configurator_proto.ItemConfigurator/DelCharacters" => {
                    #[allow(non_camel_case_types)]
                    struct DelCharactersSvc<T: ItemConfigurator>(pub Arc<T>);
                    impl<
                        T: ItemConfigurator,
                    > tonic::server::UnaryService<super::DelCharactersReq>
                    for DelCharactersSvc<T> {
                        type Response = super::DelCharactersRep;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DelCharactersReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).del_characters(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DelCharactersSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: ItemConfigurator> Clone for ItemConfiguratorServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ItemConfigurator> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ItemConfigurator> tonic::server::NamedService for ItemConfiguratorServer<T> {
        const NAME: &'static str = "item_configurator_proto.ItemConfigurator";
    }
}
